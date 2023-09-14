$ErrorActionPreference = "Stop"

# Define app-specific details
$name = "gitnr"
$binary="$name.exe"
$version="latest"
$githubRepo="reemus-dev/$name"
$downloadBaseUrl="https://github.com/$githubRepo/releases/download/$version"

if ($version -eq "latest") {
  # The latest version is accessible from a slightly different URL
  $downloadBaseUrl="https://github.com/$githubRepo/releases/latest/download"
}

# Determine system architecture
$type = (Get-ComputerInfo).CsSystemType.ToLower()
if ($type.StartsWith("x64")) {
    $downloadFile = "$name-win-amd64.exe"
} else {
    Write-Host "[Error]" -ForegroundColor Red
    Write-Host "Unsupported Architecture: $type" -ForegroundColor Red
    [Environment]::Exit(1)
}

# Create app directory
$destDir = "$env:USERPROFILE\AppData\Local\$name"
$destBin = "$destDir\$binary"
Write-Host "Creating Install Directory" -ForegroundColor White
Write-Host " $destDir"
if (-Not (Test-Path $destDir)) {
    New-Item -ItemType Directory -Path $destDir
}

# Download the executable
$downloadUrl = "$downloadBaseUrl/$downloadFile"
Write-Host "Downloading Binary" -ForegroundColor White
Write-Host " From: $downloadUrl"
Write-Host " Path: $destBin"
Invoke-WebRequest -Uri $downloadUrl -OutFile "$destBin"

# Add to user PATH if not already present
$currentPath = [System.Environment]::GetEnvironmentVariable('Path', [System.EnvironmentVariableTarget]::User)
if (-Not ($currentPath -like "*$destDir*")) {
    Write-Host "Adding Install Directory To System Path" -ForegroundColor White
    Write-Host " $destBin"
    [System.Environment]::SetEnvironmentVariable('Path', "$currentPath;$destDir", [System.EnvironmentVariableTarget]::User)
}


Write-Host "Installation Complete" -ForegroundColor Green
Write-Host " Restart your shell to starting using '$binary'. Run '$binary --help' for more information"