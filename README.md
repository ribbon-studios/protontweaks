[![Coveralls][coveralls-image]][coveralls-url] [![CI Build][github-actions-image]][github-actions-url] [![Maintainability][maintainability-image]][maintainability-url]

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
              imports = [
                inputs.protontweaks.nixosModules.protontweaks
              ];

              nixpkgs = {
                overlays = [
                  inputs.protontweaks.overlay
                ];
              };

              # This will install protontweaks and enable the watch service that automatically updates the launch options
              services.protontweaks.enable = true;
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
  run    [experimental]: Runs the steam launch command and applies any necessary tweaks
  watch  [experimental]: Watches for any steam apps to be installed and automatically adds 'protontweaks' to the launch options
  help   Print this message or the help of the given subcommand(s)

Arguments:
  [COMMAND_ARGS]...  The steam launch command '%command%'

Options:
  -h, --help     Print help
  -V, --version  Print version
```

[github-actions-image]: https://img.shields.io/github/actions/workflow/status/rain-cafe/protontweaks/ci.yml?event=push
[github-actions-url]: https://github.com/rain-cafe/protontweaks/actions/workflows/ci.yml?query=branch%3Amain
[coveralls-image]: https://img.shields.io/coveralls/rain-cafe/protontweaks.svg
[coveralls-url]: https://coveralls.io/github/rain-cafe/protontweaks?branch=main
[maintainability-image]: https://img.shields.io/codeclimate/maintainability/rain-cafe/protontweaks
[maintainability-url]: https://codeclimate.com/github/rain-cafe/protontweaks/maintainability
