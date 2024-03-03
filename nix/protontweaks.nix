{ pkgs, lib, fetchFromGitHub, rustPlatform, pkg-config, openssl }:

let
  version = "0.1.0";
in
rustPlatform.buildRustPackage {
  pname = "protontweaks";
  version = version;

  src = ../.;

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  checkFlags = [ ];

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
