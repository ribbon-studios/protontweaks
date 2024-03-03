> [!IMPORTANT]  
> This is the Protontweaks CLI repository, you can find the `protontweaks` db [over here](https://github.com/rain-cafe/protontweaks-db)

## Protontweaks

> Automatically apply necessary protontricks to your games!

### Requirements

- `protontricks` or `nix-shell`
  - If `nix-shell` is installed then we'll utilize it to temporarily install `protontricks`!

### Installation

We provide a [binary and deb package](https://github.com/rain-cafe/protontweaks/releases/latest) for installation on platforms that don't have nix.

<details>
  <summary>NixOS Flake Example</summary>

```nix
{
  description = "NixOS Example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    protontweaks.url = "github:rain-cafe/protontweaks/main";
    protontweaks.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, ... } @ inputs: {
    nixosConfigurations =
      let
        inherit (self) outputs;
        inherit (nixpkgs.lib) nixosSystem;
      in
      {
        your-hostname = nixosSystem {
          specialArgs = { inherit inputs outputs; };

          modules = [
            # This is just an example, you can obviously format this however you want!
            ({ pkgs, ... }: {
              nixpkgs = {
                overlays = [
                  inputs.protontweaks.overlay
                ];
              };

              environment.systemPackages = with pkgs; [
                protontweaks
              ];
            })
            # Your NixOS Modules here...
            ../defaults/configuration.nix
          ];
        };
      };
  };
}
```

</details>

### Usage

```sh
# Applies tweaks to all apps
protontweaks
# Applies tweaks to one app
protontweaks <app-id>
```

### TODO

- **Convenience**
  - Add the ability to install a watch service that will detect games being installed to steam
- **Performance**
  - Apply tweaks async so multiple games can be tweaked at once
  - Tweak DB Caching
