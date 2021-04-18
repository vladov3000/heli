#!/usr/bin/env bash

cargo build --release
mv target/release/heli /usr/local/bin
