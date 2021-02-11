## paswitch-rs

Wrapper around paswitch to allow for swapping to a pulse source by an attribute other than its id.

## Prerequisites

    Rust 2018
    pactl
    paswitch

## Installation

    cargo install --git https://github.com/RobertPlant/paswitch-rs --branch master

## Examples:

Swap all output to a named device:

    paswitch-rs Fiio

List available devices:

    paswitch-rs --list
