#!/bin/bash

set -Cue -o pipefail

cd $(readlink -f $(dirname $0))

if ! command -v target-gen &> /dev/null; then
    echo "Error: target-gen is missing"
    echo "To install it, run the command below:"
    echo "  cargo install --git https://github.com/probe-rs/probe-rs.git --tag v0.23.0 target-gen"
fi

cargo build --release

target-gen elf -u target/thumbv7em-none-eabihf/release/monazite_bank1    chip-description.yaml
target-gen elf -u target/thumbv7em-none-eabihf/release/monazite_bank2    chip-description.yaml
target-gen elf -u target/thumbv7em-none-eabihf/release/monazite_mirrored chip-description.yaml
