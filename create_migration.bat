@echo off
setlocal enabledelayedexpansion

:: Escolher tipo de operação: U ou V
set /p TYPE=Migration type: ([U] or [V]): 
if /I "%TYPE%"=="U" (
    set PREFIX=U
) else if /I "%TYPE%"=="V" (
    set PREFIX=V
) else (
    echo Invalid type. Use U or V only.
    exit /b
)

:: Escolher extensão
set /p EXT=Select the file extension (.sql or .rs): 
if "%EXT%"==".sql" (
    set EXTENSION=sql
) else if "%EXT%"==".rs" (
    set EXTENSION=rs
) else (
    echo Invalid extension. Use .sql or .rs only.
    exit /b
)

:: Gerar timestamp
for /f %%a in ('powershell -command "Get-Date -Format yyyyMMddHHmmss"') do set TIMESTAMP=%%a

:: Nome do arquivo
set /p NAME=Type the file name (no spaces): 

:: Caminho do diretório
set /p DIR=Enter the directory where the file should be created: 

:: Verificar se o diretório existe
if not exist "%DIR%" (
    echo The directory "%DIR%" does not exist.
    exit /b
)

:: Criar o arquivo no diretório especificado
set FILENAME=%PREFIX%%TIMESTAMP%__%NAME%.%EXTENSION%
set FULLPATH=%DIR%\%FILENAME%

echo Creating the file: %FULLPATH%
type nul > "%FULLPATH%"

echo File successfully created: %FULLPATH%
