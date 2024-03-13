{ lib, config, pkgs, ... }:
let
  cfg = config.services.protontweaks;

  inherit (lib) mkIf mkEnableOption;
in
{
  options.services.protontweaks = {
    enable = mkEnableOption "protontweaks";
  };

  config = mkIf (cfg.enable) {
    environment.systemPackages = with pkgs; [
      protontweaks
      protontricks # Install this for now until we figure out why nix-shell isn't working
    ];
  };
}
