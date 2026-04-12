use anyhow::{Context, Result};
use serde::Deserialize;

const ZKILL_BASE: &str = "https://zkillboard.com/api";

pub struct ZkillClient {
    agent: ureq::Agent,
}

impl ZkillClient {
    pub fn new() -> Self {
        let agent = ureq::Agent::new_with_defaults();
        Self { agent }
    }

    pub fn get_system_kills(&self, system_id: i64, _hours: u32) -> Result<SystemKills> {
        let url = format!("{}/kills/solarSystemID/{}/", ZKILL_BASE, system_id);
        let mut response = self
            .agent
            .get(&url)
            .header("User-Agent", "neocom/0.1")
            .header("Accept-Encoding", "gzip")
            .call()
            .with_context(|| "Failed to fetch system kills")?;
        response
            .body_mut()
            .read_json()
            .context("Failed to parse kills")
    }

    pub fn get_character_info(&self, character_id: i64) -> Result<CharacterInfo> {
        let url = format!("{}/kills/characterID/{}/", ZKILL_BASE, character_id);
        let mut response = self
            .agent
            .get(&url)
            .header("User-Agent", "neocom/0.1")
            .header("Accept-Encoding", "gzip")
            .call()
            .with_context(|| "Failed to fetch character info")?;
        response
            .body_mut()
            .read_json()
            .with_context(|| "Failed to parse character info")
    }
}

#[derive(Debug, Deserialize)]
pub struct SystemKills {
    #[serde(rename = "killCount")]
    pub kill_count: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CharacterInfo {
    pub character: Option<i64>,
    pub kills: Option<i64>,
    pub losses: Option<i64>,
    pub info: Option<CharacterDetails>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CharacterDetails {
    pub name: Option<String>,
    pub corp_id: Option<i64>,
    pub corp_name: Option<String>,
    pub alliance_id: Option<i64>,
    pub alliance_name: Option<String>,
    pub sec_status: Option<f64>,
}
