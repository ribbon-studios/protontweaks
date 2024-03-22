{ lib, config, pkgs, ... }:
let
  cfg = config.services.protontweaks;

  inherit (lib) mkIf mkEnableOption;
in
{
  options.services.protontweaks = {
    enable = mkEnableOption "protontweaks";

    gamemode = mkEnableOption "automatic gamemode initialization" // {
      default = true;
    };

    mangohud = mkEnableOption "automatic mangohud initialization" // {
      default = false;
    };
  };

  config = mkIf (cfg.enable) {
    home.file.".config/protontweaks.json".text = builtins.toJSON ({
      gamemode = cfg.gamemode;
      mangohud = cfg.mangohud;
    });
  };
}
