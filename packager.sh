#!/bin/bash

read -p "Current version: " version
rm -rf ~/.tmp/soccer-release/
mkdir -p ~/.tmp/soccer-release/
cargo build --release
cp target/release/soccer ~/.tmp/soccer-release/soccer
cp LICENSE ~/.tmp/soccer-release/LICENSE

cd ~/.tmp/soccer-release/
tar -czf soccer-v${version}-bin-linux-x86_64.tar.gz soccer LICENSE

