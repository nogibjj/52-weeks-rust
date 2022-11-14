#!/usr/bin/env bash
# This script checks for the latest version of Rust and installs it and sets to the default version
rustup update stable
rustup default stable
#check version
rustc --version