
# PSU

A really simple command line interface for your bench power supply.

I've basically re-written this program countless times in countless languages in university and for jobs. I'm frankly rather surprised a tool like this doesn't already exist.

I made this one for my own power supply, but it's a little different from my previous implementations because it was designed to be very easy to contribute to.

## Supported power supplies

I can only support what I can test and I can only test what I have, so this list is very limited at the moment. If your supply isn't here, consider [contributing!](#contributing)

 * BK Precision 1697 (should support 1696 and 1698 as well, but those are untested)

## Installation

PSU can be installed through [crates.io](https://crates.io/). You can run `cargo install psu` to get it.
It is also available as a Nix derivation.

## Usage

Before you can use this tool you need to configure it, since it can't just magically discover your power supply (at the moment).

You'll need to put a config file under `~/.config/bench_psu_config.yaml`. On Windows that'll just be a hidden folder in your home directory as well.

You will need to provide the following:
 * default_interface - the name of the power supply that should be used by default.
 * power_supplies - this is a list of power supplies you have configured your machine to use. It's a map of power supply names to their config.

Your config file may look like the following:
```yaml
default_supply: bk_precision
power_supplies:
  bk_precision:
    !bk_precision_196x
      serial_interface: /dev/serial/by-id/usb-1453_4026-if00-port0
```

Each power supply's configuration is unique. You can find their individual documentation under [power supply modules](https://docs.rs/psu/power_supplies/index.html).

## Contributing

I did my best to try and make this easy to contribute to, but feedback on how to streamline the process is welcome.

All files you add or modify need the [license header](https://www.gnu.org/licenses/gpl-howto.en.html). You can copy and update it from the `main.rs` file.

90% of contributions are likely to be drivers for new power supplies, since I can't add support for power supplies I don't have. Start by adding a power supply to the [power_supplies](src/power_supplies) folder. You'll create a submodule in that folder for your driver. See the [BK Precision 196X](src/power_supplies/bk_precision_196x.rs) driver as an example reference.
