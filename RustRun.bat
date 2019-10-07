@echo off
:loop
set proj_name=
set /p proj_name="input compile proj name;"
cargo run --manifest-path %proj_name%/Cargo.toml rem --release
goto :loop

pause
