@echo off
cd c:\rbrust\wwidocs\src\include
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\include\typeof.rs    . /D /C /Y
cd ..\settings\
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\settings\params.rs   . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\settings\config.rs   . /D /C /Y
cd ..\idocdefn\
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\idocdefn\rdparser.rs . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\idocdefn\ldtables.rs . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\idocdefn\upldmitm.rs . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\idocdefn\upldsgrp.rs . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\idocdefn\upldssgm.rs . /D /C /Y
cd ..\
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\dbcreatn.rs          . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\flat2jsn.rs       . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\idocdefn.rs          . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\main.rs              . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\sap2flat.rs       . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\settings.rs          . /D /C /Y
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\sgmalias.rs          . /D /C /Y
cd ..\target\debug
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\_config.json      . /D /C /Y
cd ..\..
xcopy c:\c-portab\01-rb\pgmfiles\wwidocs\Cargo.toml        . /D /C /Y
cargo build
pause