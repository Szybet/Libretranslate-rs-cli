#!/bin/bash

#rm -rf lib
#mkdir lib
#mkdir lib/include

cargo install cross
RUSTFLAGS='-C link-arg=-s' ~/.cargo/bin/cross build --release --target armv7-unknown-linux-musleabihf

#cargo install cbindgen
#~/.cargo/bin/cbindgen . -o lib/include/libreader-rs.h

sshpass -p root scp target/armv7-unknown-linux-musleabihf/release/Libretranslate-rs-cli root@192.168.2.2:/kobo/
