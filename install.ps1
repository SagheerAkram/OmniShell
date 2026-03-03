# OmniShell Installation Script for Windows
# Run with: powershell -ExecutionPolicy Bypass -File install.ps1

$ErrorActionPreference = "Stop"

Write-Host "================================================================" -ForegroundColor Cyan
Write-Host "           OmniShell Installation Script                        " -ForegroundColor Cyan
Write-Host "================================================================" -ForegroundColor Cyan
Write-Host ""

# OmniShell utilizes true offline Mesh networking (BLE & Wi-Fi Direct).
# No external router daemons (like Tor/I2P) are required!
Write-Host "-> Initializing native router-less environment..." -ForegroundColor Cyan
Write-Host ""


# Check for Visual Studio Build Tools
Write-Host "-> Checking for Visual Studio Build Tools..." -ForegroundColor Cyan
$vsWhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"

if (-not (Test-Path $vsWhere)) {
    Write-Host "[!] Visual Studio Build Tools not found" -ForegroundColor Yellow
    Write-Host "  Please install from: https://visualstudio.microsoft.com/downloads/" -ForegroundColor Yellow
    Write-Host "  Select 'Desktop development with C++' workload" -ForegroundColor Yellow
    Write-Host ""
    $continue = Read-Host "Continue anyway? (y/n)"
    if ($continue -ne "y") {
        exit 1
    }
}
else {
    Write-Host "[OK] Visual Studio Build Tools found" -ForegroundColor Green
}
Write-Host ""

# Check for Rust/Cargo
Write-Host "-> Checking for Rust (Cargo)..." -ForegroundColor Cyan
if (Get-Command cargo -ErrorAction SilentlyContinue) {
    Write-Host "[OK] Cargo found" -ForegroundColor Green
}
else {
    Write-Host "[!] Cargo not found. Installing Rust..." -ForegroundColor Yellow
    $rustupExe = "$env:TEMP\rustup-init.exe"
    Invoke-WebRequest -Uri "https://win.rustup.rs" -OutFile $rustupExe
    Write-Host "Running Rust installer (this may take a minute)..." -ForegroundColor Cyan
    & $rustupExe -y --default-toolchain stable --profile minimal
    
    # Add cargo to current session path temporarily to allow build
    $env:Path += ";$env:USERPROFILE\.cargo\bin"
    if (Get-Command cargo -ErrorAction SilentlyContinue) {
        Write-Host "[OK] Rust/Cargo installed successfully" -ForegroundColor Green
    }
    else {
        Write-Host "[ERROR] Failed to install Rust" -ForegroundColor Red
        exit 1
    }
}
Write-Host ""

# Build OmniShell
Write-Host "-> Building OmniShell..." -ForegroundColor Cyan
cargo build --release

if ($LASTEXITCODE -eq 0) {
    Write-Host "[OK] Build complete" -ForegroundColor Green
}
else {
    Write-Host "[ERROR] Build failed" -ForegroundColor Red
    exit 1
}
Write-Host ""

# Install binary
Write-Host "-> Installing binary..." -ForegroundColor Cyan
$installDir = "$env:USERPROFILE\.omnishell\bin"
New-Item -ItemType Directory -Force -Path $installDir | Out-Null

Copy-Item "target\release\omnishell.exe" -Destination "$installDir\omnishell.exe" -Force
Write-Host "[OK] Binary installed to $installDir\omnishell.exe" -ForegroundColor Green
Write-Host ""

# Add to PATH
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($currentPath -notlike "*$installDir*") {
    Write-Host "-> Adding to PATH..." -ForegroundColor Cyan
    [Environment]::SetEnvironmentVariable(
        "Path",
        "$currentPath;$installDir",
        "User"
    )
    $env:Path += ";$installDir"
    Write-Host "[OK] Added to PATH" -ForegroundColor Green
    Write-Host "[!] Please restart your terminal for PATH changes to take effect" -ForegroundColor Yellow
}
else {
    Write-Host "[OK] Already in PATH" -ForegroundColor Green
}
Write-Host ""

# Initialize OmniShell
Write-Host "-> Initializing OmniShell..." -ForegroundColor Cyan
& "$installDir\omnishell.exe" init
Write-Host ""

# Spawn Silent Background Mesh Daemon
Write-Host "-> Spawning Silent Background Mesh Node..." -ForegroundColor Cyan
Start-Process -FilePath "$installDir\omnishell.exe" -ArgumentList "daemon" -WindowStyle Hidden
Write-Host "[OK] OmniShell Daemon is routing offline traffic in the background." -ForegroundColor Green
Write-Host ""

Write-Host "================================================================" -ForegroundColor Cyan
Write-Host "           Installation Complete!                               " -ForegroundColor Cyan
Write-Host "================================================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "[OK] OmniShell is now installed!" -ForegroundColor Green
Write-Host ""
Write-Host "Quick Start:" -ForegroundColor White
Write-Host "  omnishell whoami          # View your identity"
Write-Host "  omnishell add <name> <key> # Add a contact"
Write-Host "  omnishell msg @alice `"Hi!`" # Send a message"
Write-Host "  omnishell help            # Show all commands"
Write-Host ""
Write-Host "Documentation:" -ForegroundColor White
Write-Host "  README.md     - Overview and features"
Write-Host "  INSTALL.md    - Detailed installation guide"
Write-Host "  COMMANDS.md   - Command reference"
Write-Host "  SECURITY.md   - Security whitepaper"
Write-Host ""
Write-Host "[!] Remember to restart your terminal!" -ForegroundColor Yellow
Write-Host ""
