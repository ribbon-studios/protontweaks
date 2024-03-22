[![Coverage][coverage-image]][coverage-url] [![CI Build][github-actions-image]][github-actions-url]

> [!IMPORTANT]  
> This is the Protontweaks CLI repository, you can find its sister repositories at the following links!

- [CLI](https://github.com/rain-cafe/protontweaks) _(you are here)_
- [App](https://github.com/rain-cafe/protontweaks-ui)
- [Database / API](https://github.com/rain-cafe/protontweaks-db)

## Protontweaks

> Automatically apply necessary protontricks to your games!

### Requirements

- [`protontricks`](https://github.com/Matoking/protontricks)

### Installation

This will automatically install protontweaks with your systems package manager if its available

```sh
$ bash -c "$(curl -fsSL https://protontweaks.com/install.sh)"
```

#### NixOS

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

#### Automatic

This installs a systemd service that automatically updates your launch options when you install a game.

```sh
$ protontweaks service --install
```

#### Manual

Add the following to the launch options for a steam game!

```sh
protontweaks %command%
```

### Configuration

We attempt to load a configuration file from the following locations, from highest to lowest priority:

1. `$XDG_CONFIG_HOME/protontweaks.json`
2. `$HOME/.config/protontweaks.json`
3. `/etc/protontweaks.json`

```jsonc
{
  // Whether to automatically run games with gamemoderun (if installed and supported)
  // Default: true
  "gamemode": true,
  // Whether to automatically run games with mangohud (if installed and supported)
  // Default: false
  "mangohud": true
}
```

[github-actions-image]: https://img.shields.io/github/actions/workflow/status/rain-cafe/protontweaks/ci.yml?event=push
[github-actions-url]: https://github.com/rain-cafe/protontweaks/actions/workflows/ci.yml?query=branch%3Amain
[coverage-image]: https://img.shields.io/codecov/c/github/rain-cafe/protontweaks
[coverage-url]: https://app.codecov.io/gh/rain-cafe/protontweaks
[maintainability-image]: https://img.shields.io/codeclimate/maintainability/rain-cafe/protontweaks
[maintainability-url]: https://codeclimate.com/github/rain-cafe/protontweaks/maintainability
