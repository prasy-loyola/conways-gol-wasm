#!/bin/sh

cargo build --target wasm32-unknown-unknown --release

cp target/wasm32-unknown-unknown/release/wasm_game.wasm .

