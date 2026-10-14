@echo off
setlocal
echo [Rust Runner] Mempersiapkan environment...

:: Tambahkan cargo bin ke PATH sesi saat ini jika belum ada
set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"

:: Cek apakah rustc tersedia
where rustc >nul 2>nul
if %errorlevel% neq 0 (
    echo [Error] rustc tidak ditemukan dalam PATH. Pastikan Rust terpasang.
    pause
    exit /b 1
)

set TARGET_FILE=main.rs
if not "%~1"=="" set TARGET_FILE=%~1

if not exist "%TARGET_FILE%" (
    echo [Error] File %TARGET_FILE% tidak ditemukan!
    pause
    exit /b 1
)

echo [Rust Runner] Meng-compile %TARGET_FILE%...
rustc "%TARGET_FILE%" -o app.exe

if %errorlevel% equ 0 (
    echo [Rust Runner] Menjalankan program:
    echo ----------------------------------------
    app.exe
    echo ----------------------------------------
    echo [Rust Runner] Program selesai dijalankan.
) else (
    echo [Error] Gagal meng-compile program.
)

pause
