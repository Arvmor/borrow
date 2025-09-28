use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

/// The base URL for the Fluid API
const API_FLUID_URL: &str = "https://api.fluid.instadapp.io";

/// The token data structure
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FluidToken {
    pub address: String,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub decimals: Option<u64>,
    pub price: Option<String>,
    pub logo_url: Option<String>,
}

/// The liquidity borrow data structure
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FluidLiquidityBorrowData {
    pub borrow: String,
    pub borrow_limit: String,
    pub last_update_timestamp: String,
    pub max_borrow_limit: String,
}

/// The vault data structure
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FluidVault {
    pub id: String,
    pub borrow_token: HashMap<String, FluidToken>,
    pub total_borrow: String,
    pub total_borrow_liquidity: String,
    pub borrow_limit: String,
    pub borrowable_until_limit: String,
    pub borrowable: String,
    pub liquidity_borrow_data: HashMap<String, FluidLiquidityBorrowData>,
}

/// Get the vault by id
///
///
/// `GET /v2/{id}/vaults/{id}`
pub fn get_vault<V: Display, I: Display>(vault: V, id: I) -> anyhow::Result<FluidVault> {
    tracing::info!("Getting vault {vault} id {id}");

    // Build the URL
    let url = format!("{API_FLUID_URL}/v2/{vault}/vaults/{id}");

    // Call the API
    let response = ureq::get(&url).call()?;
    let response = response.into_body().read_json()?;

    Ok(response)
}
