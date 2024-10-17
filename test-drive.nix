{ pkgs ? import <nixpkgs> { }, ... }:
pkgs.mkShell {
  buildInputs = [
    (import ./. { pkgs = pkgs; })
  ];
}
