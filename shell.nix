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
    rustc.llvmPackages.llvm
    cargo-llvm-cov
    gcc
    rustfmt
    clippy
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
  LLVM_COV = "${pkgs.rustc.llvmPackages.llvm}/bin/llvm-cov";
  LLVM_PROFDATA = "${pkgs.rustc.llvmPackages.llvm}/bin/llvm-profdata";
  RUST_LOG = "trace";

  LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${ with pkgs; lib.makeLibraryPath [
      vulkan-loader
      xorg.libX11
      xorg.libXcursor
      xorg.libXrandr
      xorg.libXi
      pkg-config
      wayland
      libxkbcommon
      fontconfig
  ] }";
}
