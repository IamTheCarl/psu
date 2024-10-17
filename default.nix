{ pkgs ? import <nixpkgs> { }, ... }:
let
  rust = import ./rust.nix { pkgs = pkgs; };
  rust_platform = pkgs.makeRustPlatform {
    cargo = rust;
    rustc = rust;
  };

  cargo_toml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
in
rust_platform.buildRustPackage rec {
  pname = "psu";
  version = cargo_toml.package.version;

  src = ./.;

  cargoHash = "sha256-NfQHz+cHCHB8Yg7bpDLTJF2mRvIvwVCcYSDypMOJdSk=";

  propigatedBuildInputs = [
    pkgs.nix
    pkgs.openssh
  ];

  meta = with pkgs.lib; {
    description = "A really simple command line interface for your bench power supply.";
    homepage = "https://github.com/IamTheCarl/psu";
    license = licenses.gpl3;
    maintainers = with maintainers; [ "James Carl" ];
  };
}
