@echo off
cd /
cd Rust_Test
set name=
set /p name= "プロジェクト名を入力"
cargo new %name% --bin