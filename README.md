> [!IMPORTANT]  
> This is the Protontweaks CLI repository, you can find its sister repositories at the following links!

- [CLI](https://github.com/rain-cafe/protontweaks) _(you are here)_
- [App](https://github.com/rain-cafe/protontweaks-ui)
- [Database / API](https://github.com/rain-cafe/protontweaks-db)

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
$ protontweaks --help

crowdsourced protontricks

Usage: protontweaks [COMMAND_ARGS]...
       protontweaks <COMMAND>

Commands:
  list   Lists the apps installed on Steam
  setup  Applies any necessary tweaks to a given game
  # This *may* work on non NixOS devices, however I have no way of actually confirming that.
  # Its still a heavy WIP and there's really no guarantee it'll work however using it will not cause any harm.
  run    [experimental]: Runs the steam launch command and applies any necessary tweaks
  # Not actually implemented
  watch  [placeholder]: Watches for any steam apps to be installed and automatically adds 'protontweaks' to the launch options
  help   Print this message or the help of the given subcommand(s)

Arguments:
  [COMMAND_ARGS]...  The steam launch command '%command%'

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### TODO

- **Convenience**
  - Add the ability to install a watch service that will detect games being installed to steam
- **Performance**
  - Apply tweaks async so multiple games can be tweaked at once
  - Tweak DB Caching
