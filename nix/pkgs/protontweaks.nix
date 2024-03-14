{ pkgs, lib, fetchFromGitHub, rustPlatform, pkg-config, openssl }:

let
  version = "0.1.0";
in
rustPlatform.buildRustPackage {
  pname = "protontweaks";
  version = version;

  src = ../../.;

  cargoLock = {
    lockFile = ../../Cargo.lock;
    # Temporary until we deploy protontweaks-api-rs to crates.io
    outputHashes = {
      "protontweaks-api-0.1.0" = "sha256-DqqJgCi+0WSvuypZ3cbjsuc10iXeSwqah/J+f3H5vtM=";
    };
  };

  # Most tests fail due to the isolation
  doCheck = false;

  buildInputs = [
    openssl
  ];

  nativeBuildInputs = [
    pkgs.rustc
    pkg-config
  ];

  meta = with lib; {
    description = "Automatically apply necessary protontricks to your games!";
    homepage = "https://github.com/rain-cafe/protontweaks";
    license = licenses.mit;
    maintainers = with maintainers; [ "cecilia-sanare" ];
  };
}
