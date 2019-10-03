@echo off
cd /
cd Rust_Test
set name=
set /p name= "input proj_name:"
cargo new %name% --bin