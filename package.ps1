# Set version in all crates
function Update-CargoToml {
  param (
    [string]$RootFolder,
    [string]$StringToFind = "0.0.0-replace-me",
    [string]$ReplacementString
  )

  if (-not (Test-Path $RootFolder -PathType Container)) {
    Write-Error "The specified root folder '$RootFolder' does not exist."
    return
  }

  if ([string]::IsNullOrWhiteSpace($ReplacementString)) {
    Write-Warning "No replacement string provided. No changes will be made."
    return
  }

  Get-ChildItem -Path $RootFolder -Recurse -Filter "Cargo.toml" | ForEach-Object {
    $filePath = $_.FullName
    Write-Host "Processing: $filePath"

    try {
      $content = Get-Content -Path $filePath -Raw

      if ($content -like "*$StringToFind*") {
        $newContent = $content -replace [regex]::Escape($StringToFind), $ReplacementString
        Set-Content -Path $filePath -Value $newContent -Force

        Write-Host "  Replaced '$StringToFind' with '$ReplacementString' in $filePath" -ForegroundColor Green
      }
      else {
        Write-Host "  '$StringToFind' not found in $filePath. No changes made." -ForegroundColor Yellow
      }
    }
    catch {
      Write-Error "Error processing file '$filePath': $($_.Exception.Message)"
    }
  }
}

$version = (Get-Content "./.version")

Update-CargoToml -RootFolder "." -ReplacementString $version

# Bulid
./build.ps1

# Packaging Steps,
Set-Location dist

Copy-Item -Path ../.build -Destination ./.build -Recurse > $null

# Copy Templates
New-Item -Path templates -ItemType Directory > $null
Copy-Item -Path ../templates/* -Destination ./templates/ -Recurse > $null

# TODO: Copy Packages
New-Item -Path packages -ItemType Directory > $null

Set-Location ..

Compress-Archive -Path dist/* -DestinationPath $($env:BUILD_TARGET).zip