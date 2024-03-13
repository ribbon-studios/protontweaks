{
  description = "Protontweaks";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs, ... }:
    let
      inherit (nixpkgs) lib legacyPackages;
      forAllSystems = lib.genAttrs lib.systems.flakeExposed;
    in
    {
      nixosModules =
        let
          protontweaks = import ./nix/nixos.nix;
        in
        {
          protontweaks = protontweaks;
          default = protontweaks;
        };
      # deprecated in Nix 2.8
      nixosModule = self.nixosModules.default;

      overlay = final: prev: {
        protontweaks = prev.callPackage ./nix/pkgs/protontweaks.nix { };
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
