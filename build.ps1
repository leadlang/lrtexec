$release = $env:RELEASE -eq "true"

Set-Location dialog

# Determine build type
$buildType = if ($release) { "release" } else { "debug" }
$nogui = if ($env:GUI -eq "false") { "nogui" } else {}

# Build dialog crate
if ($env:GUI -ne "false") {
  Write-Host "Building dialog ($($buildType))"
  ../compiler $buildType --lib
  ""
}

# Navigate to lrt directory
Set-Location ../lrt

# Build lrt crate
Write-Host "Building lrt ($($buildType))"
../compiler $buildType --bins $($nogui)
""

Set-Location ../smelt

# Build smelt
Write-Host "Building smelt ($($buildType))"
../compiler $buildType --bins
""

Set-Location ..

Remove-Item -Path dist -Recurse -ErrorAction SilentlyContinue
New-Item -Path dist -ItemType Directory -ErrorAction SilentlyContinue > $null

if ($env:GUI -ne "false") {
  New-Item -Path dist/dialog -ItemType Directory -ErrorAction SilentlyContinue > $null
}

$buildTarget = $env:BUILD_TARGET
$buildTarget = if ($buildTarget) { "/$($buildTarget)" } else {}

# Copy smelt.exe
Copy-Item -Path "./smelt/target$($buildTarget)/$($buildType)/*" -Filter smelt* -Destination "./dist/" -Recurse

# Copy the lrt
Copy-Item -Path "./lrt/target$($buildTarget)/$($buildType)/*" -Filter lrt* -Destination "./dist/" -Recurse

if ($env:GUI -ne "false") {
  Copy-Item -Path "./dialog/target$($buildTarget)/$($buildType)/*" -Filter intl_dialog* -Destination "./dist/dialog/" -Recurse
  Copy-Item -Path "./dialog/target$($buildTarget)/$($buildType)/*" -Filter libintl_dialog* -Destination "./dist/dialog/" -Recurse
}