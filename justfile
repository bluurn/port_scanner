set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

default:
    just --list

fmt:
    cargo fmt
    nix fmt

check:
    cargo check
    cargo clippy -- -D warnings
    nix flake check

test:
    cargo test

build:
    cargo build

release:
    cargo build --release

run *args:
    cargo run -- {{args}}

install-local:
    cargo install --path .

nix-run *args:
    nix run . -- {{args}}

nix-build:
    nix build

clean:
    cargo clean
    rm -rf result result-*

