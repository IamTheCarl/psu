{ pkgs ? import <nixpkgs> {}, ...}:
let
  rust-overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/stable.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ rust-overlay ]; };
in
pkgs.rust-bin.stable.latest.default.override {
  extensions = [
    "rust-src"
    "rust-analyzer"
    "rustfmt"
    "clippy"
  ];
  targets = [
    "x86_64-unknown-linux-gnu"
    "wasm32-unknown-unknown"
  ];
}