develop:
ifeq ($(shell printenv IN_NIX_SHELL),)
	@nix-shell --impure --run $(shell printenv SHELL)
else
	$(info You are already running in a nix shell!)
endif

validate:
	killall steam && sleep 5 || true
	nix build
	nix shell --command steam

validate-appimage:
	cargo appimage
	appimage-run ./target/appimage/protontweaks.AppImage --version
