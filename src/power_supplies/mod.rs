// Copyright James Carl (C) 2023
// GPL-3.0-or-later

//! Provides drivers for power supplies.
//!
//! Individual drivers will be provided through sub-modules.
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;

// All our currently supported power supplies.
mod bk_precision_196x;

/// A generic representation of a power supply.
/// If one of these features is not supported by the power supply you are implementing, call `bail!("Power supply does not support voltage limits")` to report
/// an error tot he user. Oh yeah, and change that message to be more appropriate to your specific instance.
#[async_trait]
pub trait PowerSupply {
    /// Enables the output without changing the current voltage/current limit settings.
    async fn enable_output(&mut self, enabled: bool) -> Result<()>;

    /// Sets the voltage limit.
    async fn set_voltage_limit(&mut self, voltage: f32) -> Result<()>;

    /// Sets the current limit.
    async fn set_current_limit(&mut self, current: f32) -> Result<()>;

    /// Terminates our connection with the power supply.
    /// This should not affect the state of the power supply's output in any way.
    async fn close(mut self: Box<Self>) -> Result<()>;
}

/// Power supply configuration that can be deserialized from the main config file.
/// Add your power supply to this.
///
/// Please do not use named fields. Put your configuration into a struct inside your own module.
/// Once you've added that, go see the `PowerSupplyConfig::load()` function. You need to do some work there too.
#[derive(Deserialize)]
pub enum PowerSupplyConfig {
    #[serde(rename = "bk_precision_196x")]
    BkPrecision196X(bk_precision_196x::Config),
}

impl PowerSupplyConfig {
    /// Takes the config we loaded and actually opens up a connection to the power supply.
    /// You need to add a call to your config to load.
    /// Please put the logic inside of a function within your module, not here.
    pub async fn load(&self) -> Result<Box<dyn PowerSupply>> {
        match self {
            PowerSupplyConfig::BkPrecision196X(config) => config.load(),
        }
        .await
    }
}
