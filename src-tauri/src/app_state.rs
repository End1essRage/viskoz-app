use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use crate::types::*;
use std::sync::{Arc, RwLock};

const KEYRING_SERVICE: &str = "com.viskoz.app"; // замени на свой bundle id
const KEYRING_USER: &str = "refresh_cookie";

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Session {
    pub is_logged_in: bool,
    pub username: String,
    access_token: String,
}

#[derive(Clone)]
pub struct AppState {
    pub client: reqwest::Client,
    pub base_url: String,
    pub cp_route: String,
    pub auth_route: String,
    // Access-токен живёт только в памяти процесса, на диск никогда не пишется.
    pub session: Arc<RwLock<Session>>,
}

impl AppState {
    pub fn new(
        base_url: impl Into<String>,
        cp_route: impl Into<String>,
        auth_route: impl Into<String>,
    ) -> Self {
        Self {
            client: reqwest::Client::builder().build().unwrap(),
            base_url: base_url.into(),
            cp_route: cp_route.into(),
            auth_route: auth_route.into(),
            session: Arc::new(RwLock::new(Session::default())),
        }
    }

    fn access_token(&self) -> String {
        self.session.read().unwrap().access_token.clone()
    }

    // --- refresh_cookie в OS secure storage (Keychain/Credential Manager/Secret Service) ---

    fn save_refresh_cookie(&self, cookie_str: &str) -> Result<()> {
        if cookie_str.is_empty() {
            return Ok(());
        }
        let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER)?;
        entry.set_password(cookie_str)?;
        Ok(())
    }

    fn get_refresh_cookie(&self) -> Option<String> {
        let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER).ok()?;
        entry.get_password().ok()
    }

    fn clear_refresh_cookie(&self) {
        if let Ok(entry) = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER) {
            let _ = entry.delete_credential();
        }
    }

    // --- auth flow ---

    pub async fn sign_up(&self, email: String, username: String, password: String) -> Result<()> {
        let url = format!("{}{}/api/v1/auth/signup", self.base_url, self.auth_route);
        let resp = self
            .client
            .post(&url)
            .json(&serde_json::json!({ "email": email, "username": username, "password": password }))
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(anyhow!("Ошибка регистрации: {}", resp.status()));
        }
        Ok(())
    }

    pub async fn login(&self, username: String, password: String) -> Result<serde_json::Value> {
        let url = format!("{}{}/api/v1/auth/login", self.base_url, self.auth_route);
        let resp = self
            .client
            .post(&url)
            .json(&serde_json::json!({ "username": username, "password": password }))
            .send()
            .await?;

        // забираем status и headers ДО любых consuming-вызовов (text()/json()),
        // чтобы не бороться с порядком заимствований
        let status = resp.status();
        let headers = resp.headers().clone();

        if !status.is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Ошибка авторизации: {} ({})", status, err_text));
        }

        let cookies: Vec<String> = headers
            .get_all(reqwest::header::SET_COOKIE)
            .iter()
            .filter_map(|h| h.to_str().ok().map(|s| s.to_string()))
            .collect();
        let cookie_string = cookies.join("; ");

        let json_resp: serde_json::Value = resp.json().await?;
        let access_token = json_resp
            .get("access_token")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let returned_username = json_resp
            .get("username")
            .and_then(|v| v.as_str())
            .unwrap_or(&username)
            .to_string();

        self.save_refresh_cookie(&cookie_string)?;

        {
            let mut session = self.session.write().unwrap();
            session.is_logged_in = true;
            session.username = returned_username.clone();
            session.access_token = access_token;
        }

        Ok(serde_json::json!({ "username": returned_username }))
    }

    /// Восстановление сессии при старте приложения: берём refresh cookie
    /// из keyring, обмениваем на новый access_token.
    pub async fn restore_session(&self) -> Result<serde_json::Value> {
        let Some(refresh_cookie) = self.get_refresh_cookie() else {
            return Ok(serde_json::json!({ "isLoggedIn": false }));
        };

        let url = format!("{}{}/api/v1/auth/refresh", self.base_url, self.auth_route);
        let resp = self
            .client
            .post(&url)
            .header("Cookie", refresh_cookie)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            self.clear_refresh_cookie();
            return Ok(serde_json::json!({ "isLoggedIn": false }));
        }

        let json_resp: serde_json::Value = resp.json().await?;

        let access_token = json_resp
            .get("access_token")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let username = json_resp
            .get("username")
            .and_then(|v| v.as_str())
            .unwrap_or("User")
            .to_string();

        {
            let mut session = self.session.write().unwrap();
            session.is_logged_in = true;
            session.username = username.clone();
            session.access_token = access_token;
        }

        Ok(serde_json::json!({ "isLoggedIn": true, "username": username }))
    }

    pub async fn logout(&self) -> Result<()> {
        self.clear_refresh_cookie();
        let mut session = self.session.write().unwrap();
        session.is_logged_in = false;
        session.username = String::new();
        session.access_token = String::new();
        Ok(())
    }

    // --- authenticated requests с автоматическим refresh при 401 ---

    pub async fn runner_reg(&self) -> Result<()> {
        let url = format!("{}{}/api/v1/runner-reg", self.base_url, self.auth_route);

        let build_req = |token: &str| {
            let mut b = self.client.post(&url);
            if !token.is_empty() {
                b = b.header("Authorization", format!("Bearer {}", token));
            }
            b
        };

        let resp = build_req(&self.access_token()).send().await?;

        if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
            // токен протух — пробуем обновить через refresh cookie и повторить запрос один раз
            let restored = self.restore_session().await?;
            let still_logged_in = restored
                .get("isLoggedIn")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            if !still_logged_in {
                return Err(anyhow!("Токен истёк, требуется повторная авторизация"));
            }

            let retry = build_req(&self.access_token()).send().await?;
            if !retry.status().is_success() {
                return Err(anyhow!("Ошибка доступа после обновления токена: {}", retry.status()));
            }
            return Ok(());
        }

        if !resp.status().is_success() {
            return Err(anyhow!("Ошибка доступа: {}", resp.status()));
        }
        Ok(())
    }

    pub async fn list_runners(&self) -> Result<Vec<RunnerInfo>> {
        #[derive(serde::Deserialize)]
        struct RunnersResponse {
            runners: Vec<RunnerInfo>,
        }
        let url = format!("{}{}/v1/runners", self.base_url, self.cp_route);
        let resp = self.client.get(&url).send().await?;
        if !resp.status().is_success() {
            return Err(anyhow!("control-plane вернул {} для {url}", resp.status()));
        }
        Ok(resp.json::<RunnersResponse>().await?.runners)
    }

    pub async fn list_servers(&self) -> Result<Vec<ServerInfo>> {
        let url = format!("{}{}/v1/servers", self.base_url, self.cp_route);
        let resp = self.client.get(&url).send().await?;
        if !resp.status().is_success() {
            return Err(anyhow!("control-plane вернул {} для {url}", resp.status()));
        }
        Ok(resp.json::<Vec<ServerInfo>>().await?)
    }

    pub async fn execute_on_runner(
        &self,
        runner_id: &str,
        req: &ExecuteRequest,
    ) -> Result<serde_json::Value> {
        let url = format!("{}{}/v1/dev/runners/{}/execute", self.base_url, self.cp_route, runner_id);
        let resp = self.client.post(&url).json(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(anyhow!("control-plane вернул {status} для {url}: {body}"));
        }
        Ok(resp.json::<serde_json::Value>().await?)
    }
}