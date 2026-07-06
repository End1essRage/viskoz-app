use crate::types::{RunnerInfo, ServerInfo,ExecuteRequest};
use anyhow::{anyhow, Result};

#[derive(Clone)]
pub struct RestClient {
    client: reqwest::Client,
    base_url: String,
}

impl RestClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.into(),
        }
    }

    pub async fn list_runners(&self) -> Result<Vec<RunnerInfo>> {
        #[derive(serde::Deserialize)]
        struct RunnersResponse {
            runners: Vec<RunnerInfo>,
        }
        
        let url = format!("{}/v1/runners", self.base_url);
        let resp = self.client.get(&url).send().await?;
        if !resp.status().is_success() {
            return Err(anyhow!("control-plane вернул {} для {url}", resp.status()));
        }
       Ok(resp.json::<RunnersResponse>().await?.runners)
    }

    pub async fn list_servers(&self) -> Result<Vec<ServerInfo>> {
        let url = format!("{}/v1/servers", self.base_url);
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
        let url = format!("{}/v1/dev/runners/{}/execute", self.base_url, runner_id);
        let resp = self.client.post(&url).json(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(anyhow!("control-plane вернул {status} для {url}: {body}"));
        }
        Ok(resp.json::<serde_json::Value>().await?)
    }
}
