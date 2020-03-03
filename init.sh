#!/usr/bin/env bash

apt update -y
apt install -y pkg-config libssl-dev
cargo run "$1"
