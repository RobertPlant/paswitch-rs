## paswitch-rs

Swap all pulse output to a sink matched by name, description or any other `pactl list` attribute, instead of by its id.

## Prerequisites

    Rust 2021
    pactl

## Installation

### Via git

    cargo install --git https://github.com/RobertPlant/paswitch-rs --branch master

### Via Nix

    nix run github:RobertPlant/paswitch-rs -- --list

or add `github:RobertPlant/paswitch-rs` as a flake input and use `packages.<system>.default`.

### Via crates.io

    cargo install paswitch-rs

## Examples:

Swap all output to a named device:

    paswitch-rs Fiio

List available devices:

    paswitch-rs --list

Interactively select an output device:

    paswitch-rs --interactive
