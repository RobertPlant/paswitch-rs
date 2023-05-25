## paswitch-rs

Wrapper around paswitch to allow for swapping to a pulse source by an attribute other than its id.

## Prerequisites

    Rust 2021
    pactl
    paswitch

## Installation

### Via git

    cargo install --git https://github.com/RobertPlant/paswitch-rs --branch master

### Via crates.io

    cargo install paswitch-rs

### On Archlinux

    yay paswitch-rs

## Examples:

Swap all output to a named device:

    paswitch-rs Fiio

List available devices:

    paswitch-rs --list

Interactively select an output device:

    paswitch-rs --interactive
