#!/bin/bash
cd `dirname $0`
pwd
echo 'input proj_name:'
read NAME
cargo new $NAME --bin