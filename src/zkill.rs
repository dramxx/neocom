#![allow(dead_code)]

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

/// Get total kills in a system (all locations) - uses pastSeconds for time filtering
    pub fn get_system_kills(&self, system_id: i64, hours: u32) -> Result<SystemKills> {
        let past_seconds = hours * 3600;
        
        // Try pastSeconds endpoint (works reliably)
        let url = format!(
            "{}/kills/solarSystemID/{}/pastSeconds/{}/",
            ZKILL_BASE, system_id, past_seconds
        );
        
        self.fetch_kills(&url)
    }
    
    fn fetch_kills(&self, url: &str) -> Result<SystemKills> {
        let mut response = self
            .agent
            .get(url)
            .header("User-Agent", "neocom/0.1")
            .header("Accept", "application/json")
            .call()
            .map_err(|e| anyhow::anyhow!("zkillboard fetch failed: {}", e))?;
        
        let body: serde_json::Value = response.body_mut().read_json()
            .map_err(|e| anyhow::anyhow!("failed to read JSON: {}", e))?;
        
        // Handle stats {killCount: N}
        if let Ok(stats) = serde_json::from_value::<SystemKills>(body.clone()) {
            if stats.kill_count.is_some() {
                return Ok(stats);
            }
        }
        
        // Handle killmail array - count the kills
        if let Some(arr) = body.as_array() {
            return Ok(SystemKills {
                kill_count: Some(arr.len() as i64),
            });
        }
        
        Ok(SystemKills { kill_count: Some(0) })
    }

    /// Get kills at specific locations (stargates) - filters killmails by locationID
    pub fn get_gate_kills(&self, system_id: i64, gate_ids: &[i64], hours: u32) -> Result<Vec<GateKill>> {
        // Try pastSeconds first (more reliable)
        let past_seconds = hours * 3600;
        let url = format!(
            "{}/kills/solarSystemID/{}/pastSeconds/{}/",
            ZKILL_BASE, system_id, past_seconds
        );
        
        if let Ok(kills) = self.fetch_gate_kills(&url, gate_ids) {
            return Ok(kills);
        }
        
        // Try hours query param
        let hours_url = format!(
            "{}/kills/solarSystemID/{}/?hours={}",
            ZKILL_BASE, system_id, hours
        );
        
        if let Ok(kills) = self.fetch_gate_kills(&hours_url, gate_ids) {
            return Ok(kills);
        }
        
        // Fallback: try without time filter
        let fallback_url = format!("{}/kills/solarSystemID/{}/", ZKILL_BASE, system_id);
        self.fetch_gate_kills(&fallback_url, gate_ids)
    }
    
    fn fetch_gate_kills(&self, url: &str, gate_ids: &[i64]) -> Result<Vec<GateKill>> {
        let mut response = self
            .agent
            .get(url)
            .header("User-Agent", "neocom/0.1")
            .header("Accept", "application/json")
            .call()
            .with_context(|| format!("Failed to fetch: {}", url))?;

        #[derive(Deserialize)]
        struct Killmail {
            #[serde(rename = "zkb")]
            zkb: ZkbInfo,
        }

        #[derive(Deserialize)]
        struct ZkbInfo {
            #[serde(rename = "locationID")]
            location_id: Option<i64>,
        }

        let kills: Vec<Killmail> = response
            .body_mut()
            .read_json()
            .context("Failed to parse killmails")?;

        let mut location_ids: Vec<i64> = Vec::new();
        for kill in &kills {
            if let Some(id) = kill.zkb.location_id {
                location_ids.push(id);
            }
        }

        // Count kills per gate location
        let mut gate_kills: Vec<GateKill> = Vec::new();
        for &gate_id in gate_ids {
            let count = location_ids.iter().filter(|&&id| id == gate_id).count() as i64;
            gate_kills.push(GateKill {
                gate_id,
                kill_count: count,
            });
        }
        
        Ok(gate_kills)
    }

    pub fn get_character_info(&self, character_id: i64) -> Result<CharacterInfo> {
        // zKillboard has separate kills and losses endpoints - fetch both
        let kills_url = format!("{}/kills/characterID/{}/", ZKILL_BASE, character_id);
        let losses_url = format!("{}/losses/characterID/{}/", ZKILL_BASE, character_id);
        
        let kills: Vec<serde_json::Value> = self
            .agent
            .get(&kills_url)
            .header("User-Agent", "neocom/0.1")
            .call()
            .ok()
            .map(|mut r| r.body_mut().read_json().unwrap_or_default())
            .unwrap_or_default();
            
        let losses: Vec<serde_json::Value> = self
            .agent
            .get(&losses_url)
            .header("User-Agent", "neocom/0.1")
            .call()
            .ok()
            .map(|mut r| r.body_mut().read_json().unwrap_or_default())
            .unwrap_or_default();

        Ok(CharacterInfo {
            character: Some(character_id),
            kills: Some(kills.len() as i64),
            losses: Some(losses.len() as i64),
            info: None,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct SystemKills {
    #[serde(rename = "killCount")]
    pub kill_count: Option<i64>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct GateKill {
    pub gate_id: i64,
    pub kill_count: i64,
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
