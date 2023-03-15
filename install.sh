#!/usr/bin/bash

if [ "$EUID" -ne 0 ]
  then echo "Please run as root"
  exit
fi

# Build the project
cargo build --release

BIN_PATH="./target/release/desktop-linker"

# Move the binary to /usr/bin
sudo mv $BIN_PATH /usr/bin/desktop-linker

echo "Desktop Linker has been installed!"