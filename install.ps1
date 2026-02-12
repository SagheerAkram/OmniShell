# OmniShell Installation Script for Windows
# Run with: powershell -ExecutionPolicy Bypass -File install.ps1

$ErrorActionPreference = "Stop"

Write-Host "================================================================" -ForegroundColor Cyan
Write-Host "           OmniShell Installation Script                        " -ForegroundColor Cyan
Write-Host "================================================================" -ForegroundColor Cyan
Write-Host ""

# Check for Tor
Write-Host "-> Checking for Tor..." -ForegroundColor Cyan
if (Get-Command tor -ErrorAction SilentlyContinue) {
    Write-Host "[OK] Tor found" -ForegroundColor Green
} else {
    Write-Host "[!] Tor not found. Installing..." -ForegroundColor Yellow
    
    $torUrl = "https://www.torproject.org/dist/torbrowser/13.0.9/tor-expert-bundle-13.0.9-windows-x86_64.tar.gz"
    $torZip = "$env:TEMP\tor-expert.tar.gz"
    $torDir = "$env:USERPROFILE\.omnishell\tor-bin"
    
    # Download Tor
    Invoke-WebRequest -Uri $torUrl -OutFile $torZip
    
    # Extract
    New-Item -ItemType Directory -Force -Path $torDir | Out-Null
    tar -xf $torZip -C $torDir
    
    # Add to PATH (persistent)
    $binPath = "$torDir\tor\src"
    [Environment]::SetEnvironmentVariable(
        "Path",
        [Environment]::GetEnvironmentVariable("Path", "User") + ";$binPath",
        "User"
    )
    $env:Path += ";$binPath"
    
    Write-Host "[OK] Tor installed to $binPath" -ForegroundColor Green
}

# Check for I2P
Write-Host "-> Checking for I2P..." -ForegroundColor Cyan
if (Get-Command i2prouter -ErrorAction SilentlyContinue) {
     Write-Host "[OK] I2P found" -ForegroundColor Green
} else {
    Write-Host "[!] I2P not found. Please install from https://geti2p.net/en/download" -ForegroundColor Yellow
} 
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
} else {
    Write-Host "[OK] Visual Studio Build Tools found" -ForegroundColor Green
}
Write-Host ""

# Build OmniShell
Write-Host "-> Building OmniShell..." -ForegroundColor Cyan
cargo build --release

if ($LASTEXITCODE -eq 0) {
    Write-Host "[OK] Build complete" -ForegroundColor Green
} else {
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
} else {
    Write-Host "[OK] Already in PATH" -ForegroundColor Green
}
Write-Host ""

# Initialize OmniShell
Write-Host "-> Initializing OmniShell..." -ForegroundColor Cyan
& "$installDir\omnishell.exe" init
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
