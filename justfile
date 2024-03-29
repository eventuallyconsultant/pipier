# Just manual: https://github.com/casey/just

_default:
	@just --list --unsorted



# ==================================================================================================
# ==================================================================================================
o_________________RUN_COMMANDS: _default

alias r := run
# run the main application
run:
  cargo r

# ==================================================================================================
# ==================================================================================================
o_________________DEV_COMMANDS: _default

alias c := check
# check
check:
	cargo check --workspace --tests 

alias t := test
# test all
test:
	cargo test --workspace
