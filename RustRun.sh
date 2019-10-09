#!/bin/bash
cd `dirname $0`
pwd
while :
do
read NAME
cargo run --manifest-path $NAME/Cargo.toml
done
