{ pkgs ? import <nixpkgs> { config.allowUnfree = true; } }:

with pkgs;

let
  inputs = [
    pkg-config
    openssl
    nixd
    nixpkgs-fmt
    rustc
    cargo
    gcc
    rustfmt
    clippy
    protontricks
  ];

  system = {
    "aarch64-linux" = "linux";
    "x86_64-linux" = "linux";
    "aarch64-darwin" = "darwin";
    "x86_64-darwin" = "darwin";
  }.${pkgs.stdenv.system};

  systemInputs = {
    "darwin" = with darwin.apple_sdk.frameworks; [
      libiconv
      CoreFoundation
      SystemConfiguration
    ];
    "linux" = [ ];
  }.${system};

  buildInputs = inputs ++ systemInputs;
in
pkgs.mkShell {
  packages = [ llvmPackages.bintools ];
  inherit buildInputs;

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  # RUST_LOG = "trace";
}
