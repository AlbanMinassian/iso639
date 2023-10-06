# doc: https://just.systems/man/en/
# `j --fmt --unstable`

# ------------------------------------------------------
# default
# ------------------------------------------------------
# just list commands
default:
    just -l

# ------------------------------------------------------
# test
# ------------------------------------------------------

alias t := test

# test
test:
    clear; \
    cargo test -- --test-threads=1 --nocapture
