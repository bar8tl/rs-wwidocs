@echo off
cd c:\rbrust\wwidocs-rs\src\definitn\
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs-rs\definitn\rdparser.rs . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs-rs\definitn\ldtables.rs . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs-rs\definitn\upldmitm.rs . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs-rs\definitn\upldsgrp.rs . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs-rs\definitn\upldssgm.rs . /D /C /Y
cd ..\pack\
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs-rs\pack\build.rs        . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs-rs\pack\dump.rs         . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs-rs\pack\setup.rs        . /D /C /Y
cd ..\unpack\
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs-rs\unpack\control.rs    . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs-rs\unpack\data.rs       . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs-rs\unpack\outputs.rs    . /D /C /Y
cd ..\
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs-rs\createdb.rs          . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs-rs\unpack.rs            . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs-rs\definitn.rs          . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs-rs\main.rs              . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs-rs\pack.rs              . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs-rs\settings.rs          . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs-rs\alias.rs             . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs-rs\query.rs             . /D /C /Y
cd ..\target\debug
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs-rs\_config.json         . /D /C /Y
cd ..\..
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs-rs\Cargo.toml           . /D /C /Y
cargo build
pause