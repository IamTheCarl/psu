// Copyright James Carl (C) 2023
// GPL-3.0-or-later

//! A generic driver for the BK Precision 196X lines of power supplies.
//!
//! Protocol Manual: 1696_98_programming_manual.pdf
//!
//! Configuration options:
//! * serial_interface - On Linux this will be a path to your serial interface device. You'll need to make sure you have permissions to access
//!                      the device. This usually means you'll need to be in the `dialout` group. On Windows, this will be something like `COM3`.
//! * address - If not specified, this will be set to 0. This is the address of the power supplies, only necessary if you changed the address of your power supply.
//!
//! Example configuration:
//! ```yaml
//! default_supply: bk_precision
//! power_supplies:
//! bk_precision:
//!   !bk_precision_196x
//!     serial_interface: /dev/serial/by-id/usb-1453_4026-if00-port0
//! ```

use super::PowerSupply;
use anyhow::{bail, Context, Result};
use async_trait::async_trait;
use serde::Deserialize;
use std::time::Duration;
use tokio_serial::SerialPort;

/// Is contained by PowerSupplyConfig in the parent module.
/// See that enum's documentation on how to write a config like this.
#[derive(Deserialize)]
pub struct Config {
    serial_interface: String,

    #[serde(default)]
    address: u8,
}

impl Config {
    pub async fn load(&self) -> Result<Box<dyn PowerSupply>> {
        if self.address > 99 {
            bail!("Power supply address is out of range.");
        }

        log::info!("Serial interface: {:?}", self.serial_interface);
        let mut serial = tokio_serial::new(&self.serial_interface, 9600)
            .data_bits(tokio_serial::DataBits::Eight)
            .parity(tokio_serial::Parity::None)
            .stop_bits(tokio_serial::StopBits::One)
            .open()
            .context("Failed to open serial port.")?;

        // Get the power supply ready for some commands.
        Command::SESS.send(self.address, &mut serial).await?;

        Ok(Box::new(BkPrecision196X {
            serial,
            address: self.address,
        }))
    }
}

/// An interface for the BK Precision 196X family of power supplies.
///
/// Most of its functionality comes from implementing the PowerSupply trait.
struct BkPrecision196X {
    serial: Box<dyn SerialPort>,
    address: u8,
}

#[async_trait]
impl PowerSupply for BkPrecision196X {
    async fn enable_output(&mut self, enabled: bool) -> Result<()> {
        Command::SOUT { enabled }
            .send(self.address, &mut self.serial)
            .await?;

        Ok(())
    }
    async fn set_voltage_limit(&mut self, voltage: f32) -> Result<()> {
        Command::VOLT { voltage }
            .send(self.address, &mut self.serial)
            .await?;

        Ok(())
    }
    async fn set_current_limit(&mut self, current: f32) -> Result<()> {
        Command::CURR { current }
            .send(self.address, &mut self.serial)
            .await?;

        Ok(())
    }
    async fn close(mut self: Box<Self>) -> Result<()> {
        // Put the power supply back into manual control mode.
        Command::ENDS.send(self.address, &mut self.serial).await?;

        Ok(())
    }
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

        // I've found that the thing starts reading commands incorrectly if I send them too quickly.
        tokio::time::sleep(Duration::from_millis(100)).await;

        Ok(())
    }
}
