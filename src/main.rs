// Copyright James Carl (C) 2023
// GPL-3.0-or-later

use anyhow::{Context, Result};
use argh::FromArgs;

mod config;
use config::Config;

mod power_supplies;

/// Control your power supply.
#[derive(FromArgs, PartialEq, Debug)]
struct Arguments {
    #[argh(subcommand)]
    command: UserCommand,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum UserCommand {
    PowerOn(PowerOn),
    PowerOff(PowerOff),
    SetLimits(SetLimits),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Power on the power supply.
#[argh(subcommand, name = "on")]
struct PowerOn {
    #[argh(option, short = 'V')]
    /// set the voltage limit while you power it on.
    voltage_limit: Option<f32>,

    #[argh(option, short = 'I')]
    /// set the current limit while you power it on.
    current_limit: Option<f32>,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Power off the power supply.
#[argh(subcommand, name = "off")]
struct PowerOff {}

#[derive(FromArgs, PartialEq, Debug)]
/// Set the limits of the power supply without changing the on/off state.
#[argh(subcommand, name = "set")]
struct SetLimits {
    #[argh(option, short = 'V')]
    /// set the voltage limit.
    voltage_limit: Option<f32>,

    #[argh(option, short = 'I')]
    /// set the current limit.
    current_limit: Option<f32>,
}

#[tokio::main]
async fn main() {
    simple_logger::init().unwrap();
    if let Err(error) = trampoline().await {
        log::error!("Unrecoverable error: {:?}", error);
    }
}

async fn trampoline() -> Result<()> {
    let arguments: Arguments = argh::from_env();
    let config = Config::load().await?;

    let mut power_supply = config
        .get_power_supply()
        .await
        .context("Failed to prepare power supply.")?;

    match arguments.command {
        UserCommand::PowerOn(power_on) => {
            if let Some(voltage_limit) = power_on.voltage_limit {
                power_supply.set_voltage_limit(voltage_limit).await?;
            }

            if let Some(current_limit) = power_on.current_limit {
                power_supply.set_current_limit(current_limit).await?;
            }

            power_supply.enable_output(true).await?;
        }
        UserCommand::PowerOff(_) => {
            power_supply.enable_output(false).await?;
        }
        UserCommand::SetLimits(limits) => {
            if let Some(voltage_limit) = limits.voltage_limit {
                power_supply.set_voltage_limit(voltage_limit).await?;
            }

            if let Some(current_limit) = limits.current_limit {
                power_supply.set_current_limit(current_limit).await?;
            }
        }
    }

    power_supply
        .close()
        .await
        .context("Failed to close power supply interface.")?;

    Ok(())
}
