{
  description = "Firmware for robot's integration microcontroller";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url  = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    crane,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
	craneLib = crane.mkLib pkgs;
      in
      {
        devShells.default = with pkgs; craneLib.devShell {
          packages = [
	    bashInteractive
            openssl
          ];

	  shellHook = ''
            export SHELL=${pkgs.bashInteractive}/bin/bash
          '';
        };
	
	packages.default = with pkgs;
          craneLib.buildPackage {
            src = craneLib.cleanCargoSource ./.;
	  };
      }
    );
}
