#!/bin/bash

cd `dirname $0`
pwd
read NAME
cargo run --manifest-path $NAME/Cargo.toml
