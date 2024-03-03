{
  description = "Protontweaks";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { nixpkgs, ... }:
    let
      inherit (nixpkgs) lib legacyPackages;
      forAllSystems = lib.genAttrs lib.systems.flakeExposed;
    in
    {
      overlay = final: prev: {
        protontweaks = prev.callPackage ./nix/protontweaks.nix { };
      };

      packages = forAllSystems (system:
        let
          pkgs = legacyPackages.${system};
        in
        {
          default = pkgs.callPackage ./nix/protontweaks.nix { };
        });
    };
}
