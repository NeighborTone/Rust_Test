@echo off
set name=
set /p name= "input proj_name:"
cargo new %name% --bin