use anyhow::{Context, Result};
use serde::Deserialize;

const ESI_BASE: &str = "https://esi.evetech.net/latest";

pub struct EsiClient {
    agent: ureq::Agent,
}

impl EsiClient {
    pub fn new() -> Self {
        let agent = ureq::Agent::new_with_defaults();
        Self { agent }
    }

    pub fn resolve_names(&self, names: &[&str]) -> Result<Vec<ResolvedId>> {
        if names.is_empty() {
            return Ok(vec![]);
        }
        let url = format!("{}/universe/ids/", ESI_BASE);
        let body = serde_json::to_value(names)?;
        let mut response = self
            .agent
            .post(&url)
            .header("User-Agent", "neocom/0.1")
            .send_json(&body)
            .with_context(|| "Failed to resolve names")?;

        #[derive(Deserialize)]
        struct IdsResponse {
            #[serde(rename = "characters")]
            chars: Option<Vec<ResolvedId>>,
            #[serde(rename = "systems")]
            systems: Option<Vec<ResolvedId>>,
            #[serde(rename = "inventory_types")]
            types: Option<Vec<ResolvedId>>,
        }

        let resp: IdsResponse = response
            .body_mut()
            .read_json()
            .with_context(|| "Failed to parse ID response")?;
        let mut results = Vec::new();
        if let Some(c) = resp.chars {
            results.extend(c);
        }
        if let Some(s) = resp.systems {
            results.extend(s);
        }
        if let Some(t) = resp.types {
            results.extend(t);
        }
        Ok(results)
    }

    pub fn get_route(&self, origin: &str, destination: &str, flag: &str) -> Result<Vec<RouteHop>> {
        let origin_id = self
            .resolve_names(&[origin])?
            .pop()
            .with_context(|| format!("Unknown origin: {}", origin))?;
        let dest_id = self
            .resolve_names(&[destination])?
            .pop()
            .with_context(|| format!("Unknown destination: {}", destination))?;

        let url = format!(
            "{}/route/{}/{}/?flag={}",
            ESI_BASE, origin_id.id, dest_id.id, flag
        );
        let mut response = self
            .agent
            .get(&url)
            .header("User-Agent", "neocom/0.1")
            .call()
            .with_context(|| "Failed to fetch route")?;

        let system_ids: Vec<i64> = response
            .body_mut()
            .read_json()
            .with_context(|| "Failed to parse route")?;
        let mut route = Vec::new();
        for system_id in system_ids {
            let sys = self.get_system_info(system_id)?;
            route.push(RouteHop {
                system_id,
                name: sys.name,
                security: sys.security,
            });
        }
        Ok(route)
    }

    pub fn get_system_info(&self, system_id: i64) -> Result<SystemInfo> {
        let url = format!("{}/universe/systems/{}/", ESI_BASE, system_id);
        let mut response = self
            .agent
            .get(&url)
            .header("User-Agent", "neocom/0.1")
            .call()
            .with_context(|| format!("Failed to fetch system {}", system_id))?;
        response
            .body_mut()
            .read_json()
            .with_context(|| "Failed to parse system info")
    }

    /// Get stargate IDs for a system (uses cached stargates from system info)
    pub fn get_stargate_ids(&self, system_id: i64) -> Result<Vec<i64>> {
        let info = self.get_system_info(system_id)?;
        Ok(info.stargates.unwrap_or_default())
    }

    /// Get detailed stargate info (including destination system)
    pub fn get_stargate_info(&self, gate_id: i64) -> Result<StargateInfo> {
        let url = format!("{}/universe/stargates/{}/", ESI_BASE, gate_id);
        let mut response = self
            .agent
            .get(&url)
            .header("User-Agent", "neocom/0.1")
            .call()
            .with_context(|| format!("Failed to fetch stargate {}", gate_id))?;
        response
            .body_mut()
            .read_json()
            .with_context(|| "Failed to parse stargate info")
    }

    /// Resolve a system ID to its name
    pub fn resolve_system(&self, system_id: i64) -> Result<String> {
        let url = format!("{}/universe/systems/{}/", ESI_BASE, system_id);
        let mut response = self
            .agent
            .get(&url)
            .header("User-Agent", "neocom/0.1")
            .call()
            .with_context(|| format!("Failed to fetch system {}", system_id))?;

        #[derive(Deserialize)]
        struct SystemDetails {
            name: String,
        }

        let details: SystemDetails = response
            .body_mut()
            .read_json()
            .with_context(|| "Failed to parse system details")?;
        Ok(details.name)
    }

    pub fn get_market_orders(&self, type_id: i64, region_id: i64) -> Result<Vec<MarketOrder>> {
        let url = format!(
            "{}/markets/{}/orders/?type_id={}&order_type=all",
            ESI_BASE, region_id, type_id
        );
        let mut response = self
            .agent
            .get(&url)
            .header("User-Agent", "neocom/0.1")
            .call()
            .with_context(|| "Failed to fetch market orders")?;

        let orders: Vec<MarketOrder> = response
            .body_mut()
            .read_json()
            .with_context(|| "Failed to parse orders")?;
        Ok(orders)
    }

    pub fn get_status(&self) -> Result<ServerStatus> {
        let url = format!("{}/status/", ESI_BASE);
        let mut response = self
            .agent
            .get(&url)
            .header("User-Agent", "neocom/0.1")
            .call()
            .with_context(|| "Failed to fetch server status")?;
        response
            .body_mut()
            .read_json()
            .with_context(|| "Failed to parse status")
    }
}

#[derive(Debug, Deserialize)]
pub struct ResolvedId {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RouteHop {
    pub system_id: i64,
    pub name: String,
    pub security: f64,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SystemInfo {
    pub name: String,
    pub system_id: i64,
    #[serde(rename = "security_status")]
    pub security: f64,
    #[serde(rename = "region_id")]
    pub region: Option<i64>,
    pub stargates: Option<Vec<i64>>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct MarketOrder {
    #[serde(rename = "type_id")]
    pub type_id: i64,
    pub price: f64,
    #[serde(rename = "volume_remain")]
    pub volume: i64,
    #[serde(rename = "is_buy_order")]
    pub is_buy: bool,
}

#[derive(Debug, Deserialize)]
pub struct ServerStatus {
    pub players: i64,
    #[serde(rename = "server_version")]
    pub version: String,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct StargateInfo {
    pub name: String,
    pub stargate_id: i64,
    #[serde(rename = "destination")]
    pub destination: Option<Destination>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct Destination {
    pub system_id: i64,
}
