{ lib, config, pkgs, ... }:
let
  cfg = config.services.protontweaks;

  inherit (lib) mkIf mkEnableOption mkOption types;
  inherit (types) nullOr;

  protontweaksConfig = with types; submodule
    {
      options = {
        gamemode = mkEnableOption "automatic gamemode initialization" // {
          default = true;
        };

        mangohud = mkEnableOption "automatic mangohud initialization" // {
          default = false;
        };
      };
    };
in
{
  options.services.protontweaks.config = mkOption {
    description = "The protontweaks config";
    type = nullOr (protontweaksConfig);
  };

  config = mkIf (cfg.config != null) {
    home.file.".config/protontweaks.json".text = builtins.toJSON cfg.config;
  };
}
