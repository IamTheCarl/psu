use std::time::Duration;

use anyhow::{Context, Result};
use argh::FromArgs;
use serde::Deserialize;
use tokio::fs;
use tokio_serial::SerialPort;

#[derive(Deserialize)]
struct Config {
    serial_interface: String,
    address: u8,
}

/// Control your power supply.
#[derive(FromArgs, PartialEq, Debug)]
struct Arguments {
    #[argh(subcommand)]
    command: ArgumentCommand,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum ArgumentCommand {
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

    let home_directory = home::home_dir().context("Could not get user's home directory.")?;
    let config = fs::read_to_string(
        home_directory
            .join(".config")
            .join("bk_precision_interface.yaml"),
    )
    .await?;

    let config: Config =
        serde_yaml::from_str(&config).context("Failed to deserialize config file.")?;

    log::info!("Serial interface: {:?}", config.serial_interface);
    let mut serial = tokio_serial::new(config.serial_interface, 9600)
        .data_bits(tokio_serial::DataBits::Eight)
        .parity(tokio_serial::Parity::None)
        .stop_bits(tokio_serial::StopBits::One)
        .open()
        .context("Failed to open serial port.")?;

    Command::SESS.send(config.address, &mut serial).await?;

    match arguments.command {
        ArgumentCommand::PowerOn(power_on) => {
            if let Some(voltage_limit) = power_on.voltage_limit {
                Command::VOLT {
                    voltage: voltage_limit,
                }
                .send(config.address, &mut serial)
                .await?;
            }

            if let Some(current_limit) = power_on.current_limit {
                Command::CURR {
                    current: current_limit,
                }
                .send(config.address, &mut serial)
                .await?;
            }

            Command::SOUT { enabled: true }
                .send(config.address, &mut serial)
                .await?;
        }
        ArgumentCommand::PowerOff(_) => {
            Command::SOUT { enabled: false }
                .send(config.address, &mut serial)
                .await?;
        }
        ArgumentCommand::SetLimits(limits) => {
            if let Some(voltage_limit) = limits.voltage_limit {
                Command::VOLT {
                    voltage: voltage_limit,
                }
                .send(config.address, &mut serial)
                .await?;
            }

            if let Some(current_limit) = limits.current_limit {
                Command::CURR {
                    current: current_limit,
                }
                .send(config.address, &mut serial)
                .await?;
            }
        }
    }

    Command::ENDS.send(config.address, &mut serial).await?;

    Ok(())
}

const CR: &str = "\r";

#[allow(clippy::upper_case_acronyms)]
enum Command {
    SESS,
    ENDS,
    VOLT { voltage: f32 },
    CURR { current: f32 },
    SOUT { enabled: bool },
}

impl Command {
    async fn send(&self, address: u8, serial: &mut Box<dyn SerialPort>) -> Result<()> {
        let to_send = match self {
            Command::SESS => format!("SESS{:02}{}", address, CR),
            Command::ENDS => format!("ENDS{:02}{}", address, CR),
            Command::VOLT { voltage } => format!("VOLT{:02}{:03}{}", address, voltage * 10.0, CR),
            Command::CURR { current } => format!("CURR{:02}{:03}{}", address, current * 100.0, CR),
            Command::SOUT { enabled } => {
                format!(
                    "SOUT{:02}{}{}",
                    address,
                    if *enabled { "0" } else { "1" },
                    CR
                )
            }
        };

        log::trace!("SEND: {}", to_send);

        serial.write_all(to_send.as_bytes())?;
        tokio::time::sleep(Duration::from_millis(100)).await;

        Ok(())
    }
}
