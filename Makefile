develop:
ifeq ($(shell printenv IN_NIX_SHELL),)
	@nix-shell --impure --run $(shell printenv SHELL)
else
	$(info You are already running in a nix shell!)
endif
