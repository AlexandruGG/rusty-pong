#!/bin/bash

printf "\nBuilding release...\n"
cargo build --release --quiet

printf "\nPreparing archive...\n"
mkdir -p pong_release/src/resources
cp -R src/resources pong_release/src/
cp target/release/pong pong_release/pong
cp install.sh pong_release/
cp install_readme.txt pong_release/
chmod 777 pong_release
zip -rmq pong_release.zip pong_release

printf "\nArchive ready as pong_release.zip!\n"
