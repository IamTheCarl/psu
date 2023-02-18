//! Handles configuration.
use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use tokio::fs;

use crate::power_supplies::{PowerSupply, PowerSupplyConfig};

#[derive(Deserialize)]
pub struct Config {
    /// The default power supply that should be used when a specific one isn't specified.
    pub default_supply: String,
    pub power_supplies: HashMap<String, PowerSupplyConfig>,
}

impl Config {
    pub async fn load() -> Result<Self> {
        let home_directory = home::home_dir().context("Could not get user's home directory.")?;
        let config =
            fs::read_to_string(home_directory.join(".config").join("bench_psu_config.yaml"))
                .await?;

        let config: Self =
            serde_yaml::from_str(&config).context("Failed to deserialize config file.")?;

        Ok(config)
    }

    pub async fn get_power_supply(&self) -> Result<Box<dyn PowerSupply>> {
        let power_supply_name =
            std::env::var("PSU_NAME").unwrap_or_else(|_error| self.default_supply.clone());

        let power_supply = self
            .power_supplies
            .get(&power_supply_name)
            .context(format!(
                "Could not find power supply `{}` in config.",
                power_supply_name
            ))?;

        power_supply.load().await
    }
}
