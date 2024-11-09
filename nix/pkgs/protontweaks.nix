{ pkgs, lib, fetchFromGitHub, rustPlatform, pkg-config, openssl }:

let
  cargo = lib.importTOML ../../Cargo.toml;
  version = cargo.package.version;
in
rustPlatform.buildRustPackage {
  pname = "protontweaks";
  version = version;

  src = ../../.;

  cargoLock = {
    lockFile = ../../Cargo.lock;
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
    maintainers = [ "cecilia-sanare" ];
  };
}
