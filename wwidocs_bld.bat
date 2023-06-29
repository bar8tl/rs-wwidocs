@echo off
cd c:\rbrust\wwidocs\src\include
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\include\typeof.rs    . /D /C /Y
cd ..\settings\
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\settings\params.rs   . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\settings\config.rs   . /D /C /Y
cd ..\definitn\
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\definitn\rdparser.rs . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\definitn\ldtables.rs . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\definitn\upldmitm.rs . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\definitn\upldsgrp.rs . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\definitn\upldssgm.rs . /D /C /Y
cd ..\pack\
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\pack\build.rs        . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\pack\dump.rs         . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\pack\setup.rs        . /D /C /Y
cd ..\unpack\
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\unpack\control.rs    . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\unpack\data.rs       . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\unpack\outputs.rs    . /D /C /Y
cd ..\
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\createdb.rs          . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\unpack.rs            . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\definitn.rs          . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\main.rs              . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\pack.rs              . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\settings.rs          . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\alias.rs             . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\query.rs             . /D /C /Y
cd ..\target\debug
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\_config.json         . /D /C /Y
cd ..\..
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\Cargo.toml           . /D /C /Y
cargo build
pause