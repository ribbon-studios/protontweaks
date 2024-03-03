> [!IMPORTANT]  
> This is the Protontweaks CLI repository, you can find the `protontweaks` db [over here](https://github.com/rain-cafe/protontweaks-db)

## Protontweaks

> Automatically apply necessary protontricks to your games!

### Requirements

- `protontricks` or `nix-shell`
  - If `nix-shell` is installed then we'll utilize it to temporarily install `protontricks`!

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
