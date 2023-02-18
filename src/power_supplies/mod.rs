//! Provides drivers for power supplies.
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;

mod bk_precision_196x;

#[async_trait]
pub trait PowerSupply {
    async fn enable_output(&mut self, enabled: bool) -> Result<()>;
    async fn set_voltage_limit(&mut self, voltage: f32) -> Result<()>;
    async fn set_current_limit(&mut self, current: f32) -> Result<()>;
    async fn close(mut self: Box<Self>) -> Result<()>;
}

#[derive(Deserialize)]
pub enum PowerSupplyConfig {
    #[serde(rename = "bk_precision_196x")]
    BkPrecision196X(bk_precision_196x::Config),
}

impl PowerSupplyConfig {
    pub async fn load(&self) -> Result<Box<dyn PowerSupply>> {
        match self {
            PowerSupplyConfig::BkPrecision196X(config) => config.load(),
        }
        .await
    }
}
