@echo off
set proj_name=
set /p proj_name="input compile proj name;"
cd %proj_name%
cargo run

pause