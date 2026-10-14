param (
    [string]$File = "main.rs"
)

# Pastikan PATH cargo aktif di session PowerShell ini
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","User") + ";" + 
            [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + 
            "$env:USERPROFILE\.cargo\bin;" + $env:Path

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Rust Runner (PowerShell)              " -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

if (-not (Test-Path $File)) {
    Write-Host "[Error] File '$File' tidak ditemukan!" -ForegroundColor Red
    exit 1
}

$outputExe = "app.exe"
Write-Host "-> Meng-compile '$File'..." -ForegroundColor Yellow

rustc $File -o $outputExe

if ($LASTEXITCODE -eq 0) {
    Write-Host "-> Menjalankan program:`n" -ForegroundColor Green
    & ".\$outputExe"
    Write-Host "`n-> Program selesai dijalankan." -ForegroundColor Green
} else {
    Write-Host "[Error] Kompilasi gagal!" -ForegroundColor Red
}
