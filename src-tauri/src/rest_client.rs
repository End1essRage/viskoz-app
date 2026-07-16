use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

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
    pub session: Arc<RwLock<Session>>,
}

impl AppState {
    pub fn new(
        base_url: impl Into<String>,
        cp_route: impl Into<String>,
        auth_route: impl Into<String>,
    ) -> Self {
        Self {
            // cookie_store(true) можно включить, но мы будем управлять кукой вручную для персистентности
            client: reqwest::Client::builder().build().unwrap(),
            base_url: base_url.into(),
            cp_route: cp_route.into(),
            auth_route: auth_route.into(),
            session: Arc::new(RwLock::new(Session::default())),
        }
    }

    // Сохраняем ТОЛЬКО refresh_cookie на диск
    fn save_refresh_cookie(&self, app_handle: &tauri::AppHandle, cookie_str: &str) -> Result<()> {
        if cookie_str.is_empty() {
            return Ok(());
        }
        let store = app_handle
            .plugin("secure-store")
            .expect("secure-store plugin missing");
        store.run_move(|s| s.set("refresh_cookie", cookie_str.to_string()))?;
        Ok(())
    }

    fn get_refresh_cookie(&self, app_handle: &tauri::AppHandle) -> String {
        let store = app_handle
            .plugin("secure-store")
            .expect("secure-store plugin missing");
        store.run_move(|s| s.get("refresh_cookie").unwrap_or_default())
    }

    pub async fn sign_up(
        &self,
        app_handle: tauri::AppHandle,
        email: String,
        username: String,
        password: String,
    ) -> Result<()> {
        let url = format!("{}{}/api/v1/auth/signup", self.base_url, self.auth_route);
        let resp = self
            .client
            .post(&url)
            .json(&serde_json::json!({ "email": email,"username": username, "password": password }))
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(anyhow!("Ошибка регистрации: {}", resp.status()));
        }
        Ok(())
    }

    pub async fn runner_reg(&self, app_handle: tauri::AppHandle) -> Result<()> {
        let url = format!("{}{}/api/v1/runner-reg", self.base_url, self.auth_route);

        // Читаем access_token из памяти (быстро)
        let access_token = self.session.read().unwrap().access_token.clone();

        let mut req = self.client.post(&url);

        if !access_token.is_empty() {
            req = req.header("Authorization", format!("Bearer {}", access_token));
        }

        let resp = req.send().await?;

        // TODO вынести в отдельную обертку
        // Если получили 401, можно попробовать обновить токен
        if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
            // Здесь можно вызвать self.restore_session() и повторить запрос
            if !self.restore_session(app_handle).await?.is_err() {
                resp = req.send().await?;
            }

            return Err(anyhow!("Токен истек, требуется повторная авторизация"));
        }

        if !resp.status().is_success() {
            return Err(anyhow!("Ошибка доступа: {}", resp.status()));
        }

        Ok(())
    }

    pub async fn login(
        &self,
        app_handle: tauri::AppHandle,
        username: String,
        password: String,
    ) -> Result<serde_json::Value> {
        let url = format!("{}{}/api/v1/auth/login", self.base_url, self.auth_route);
        let resp = self
            .client
            .post(&url)
            .json(&serde_json::json!({ "username": username, "password": password }))
            .send()
            .await?;

        if !resp.status().is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            return Err(anyhow!(
                "Ошибка авторизации: {} ({})",
                resp.status(),
                err_text
            ));
        }

        // 1. Парсим access_token из тела ответа
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

        // 2. Вытаскиваем refresh_cookie из заголовков
        let cookies: Vec<String> = resp
            .headers()
            .get_all(reqwest::header::SET_COOKIE)
            .iter()
            .filter_map(|h| h.to_str().ok().map(|s| s.to_string()))
            .collect();
        let cookie_string = cookies.join("; ");

        // 3. Сохраняем refresh_cookie на диск (безопасно)
        self.save_refresh_cookie(&app_handle, &cookie_string)?;

        // 4. Сохраняем access_token ТОЛЬКО в памяти (в session)
        let mut session = self.session.write().unwrap();
        session.is_logged_in = true;
        session.username = returned_username.clone();
        session.access_token = access_token;

        Ok(serde_json::json!({ "username": returned_username }))
    }

    // Восстановление сессии при старте приложения
    pub async fn restore_session(&self, app_handle: tauri::AppHandle) -> Result<serde_json::Value> {
        let refresh_cookie = self.get_refresh_cookie(&app_handle);

        if refresh_cookie.is_empty() {
            return Ok(serde_json::json!({ "isLoggedIn": false }));
        }

        // Делаем запрос к /refresh с кукой
        let url = format!("{}{}/api/v1/auth/refresh", self.base_url, self.auth_route);
        let resp = self
            .client
            .post(&url)
            .header("Cookie", refresh_cookie)
            .send()
            .await?;

        if !resp.status().is_success() {
            // Если рефреш не удался (истекла кука), очищаем хранилище
            let store = app_handle
                .plugin("secure-store")
                .expect("secure-store plugin missing");
            let _ = store.run_move(|s| s.delete("refresh_cookie"));
            return Ok(serde_json::json!({ "isLoggedIn": false }));
        }

        // Получаем новый access_token
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

        // Сохраняем в памяти
        let mut session = self.session.write().unwrap();
        session.is_logged_in = true;
        session.username = username.clone();
        session.access_token = access_token;

        Ok(serde_json::json!({
            "isLoggedIn": true,
            "username": username
        }))
    }

    pub async fn logout(&self, app_handle: tauri::AppHandle) -> Result<()> {
        // Очищаем secure-store
        let store = app_handle
            .plugin("secure-store")
            .expect("secure-store plugin missing");
        let _ = store.run_move(|s| s.delete("refresh_cookie"));

        // Очищаем память
        let mut session = self.session.write().unwrap();
        session.is_logged_in = false;
        session.username = String::new();
        session.access_token = String::new();
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
        let url = format!(
            "{}{}/v1/dev/runners/{}/execute",
            self.base_url, self.cp_route, runner_id
        );
        let resp = self.client.post(&url).json(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(anyhow!("control-plane вернул {status} для {url}: {body}"));
        }
        Ok(resp.json::<serde_json::Value>().await?)
    }
}
