#!/bin/sh
cargo build --release
cp ./target/release/erau-chess-backend ./target/release/erau-chess-backend-stripped
strip ./target/release/erau-chess-backend-stripped
scp -P 56777 ./target/release/erau-chess-backend-stripped root@5.149.253.217:/etc/erauchess.org/server
