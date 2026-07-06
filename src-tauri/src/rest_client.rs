use crate::types::{RunnerInfo, ServerInfo};
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
        let url = format!("{}/api/v1/runners", self.base_url);
        let resp = self.client.get(&url).send().await?;
        if !resp.status().is_success() {
            return Err(anyhow!("control-plane вернул {} для {url}", resp.status()));
        }
        Ok(resp.json::<Vec<RunnerInfo>>().await?)
    }

    pub async fn list_servers(&self) -> Result<Vec<ServerInfo>> {
        let url = format!("{}/api/v1/servers", self.base_url);
        let resp = self.client.get(&url).send().await?;
        if !resp.status().is_success() {
            return Err(anyhow!("control-plane вернул {} для {url}", resp.status()));
        }
        Ok(resp.json::<Vec<ServerInfo>>().await?)
    }
}
