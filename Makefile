develop:
ifeq ($(shell printenv IN_NIX_SHELL),)
	@nix-shell --impure --run $(shell printenv SHELL)
else
	$(info You are already running in a nix shell!)
endif

validate:
	killall steam && sleep 5
	nix build
	nix shell --command steam
