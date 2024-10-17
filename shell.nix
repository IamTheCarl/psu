{ pkgs ? import <nixpkgs> { }, ... }:
let
  rust = import ./rust.nix { pkgs = pkgs; };
in
pkgs.mkShell {
  buildInputs = [
    rust
  ];
}
