@echo off
rem zwwidocs-rs_bld.bat - Script to start compiling of program wwidocs-rs
rem (2021-07-01 bar8tl)
cargo build
xcopy ..\target\debug\wwidocs-rs.exe ..\extras\ /D /C /Y
pause
