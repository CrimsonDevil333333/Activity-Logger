# Build and Package Script for Windows
# Run this script to create a distributable package

Write-Host "Building Activity Logger..." -ForegroundColor Cyan

# Build release version
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}

Write-Host "Build successful!" -ForegroundColor Green
Write-Host "Creating distribution package..." -ForegroundColor Cyan

# Create distribution directory
$distDir = "dist\Activity-Logger-Windows"
Remove-Item -Path $distDir -Recurse -Force -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Path $distDir -Force | Out-Null

# Copy files
Copy-Item "target\release\activity_logger.exe" -Destination $distDir
Copy-Item "config.json" -Destination $distDir
Copy-Item "run.bat" -Destination $distDir
Copy-Item "DISTRIBUTION.md" -Destination "$distDir\README.md"

# Copy icon if exists
if (Test-Path "assets\icon.ico") {
    Copy-Item "assets\icon.ico" -Destination $distDir
}

Write-Host "Package created at: $distDir" -ForegroundColor Green

# Create ZIP archive
$zipPath = "dist\Activity-Logger-Windows.zip"
Remove-Item -Path $zipPath -Force -ErrorAction SilentlyContinue

Compress-Archive -Path $distDir -DestinationPath $zipPath -Force

Write-Host "ZIP archive created: $zipPath" -ForegroundColor Green
Write-Host ""
Write-Host "Distribution package ready!" -ForegroundColor Cyan
Write-Host "You can share the ZIP file with others." -ForegroundColor Cyan
