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
Write-Host "Building lrt, lgui ($($buildType))"
../compiler $buildType --bins $($nogui)
""

Set-Location ../linst

Write-Host "Building linst ($($buildType))"
../compiler $buildType --bins
""

Set-Location ../smelt

# Build smelt
Write-Host "Building smelt ($($buildType))"
../compiler $buildType --bins
""

Set-Location ../async_waker

# Build waker
Write-Host "Building waker ($($buildType))"
../compiler $buildType --lib
""

Set-Location ..

Remove-Item -Path dist -Recurse -ErrorAction SilentlyContinue
New-Item -Path dist -ItemType Directory -ErrorAction SilentlyContinue > $null

New-Item -Path dist/libs/waker -ItemType Directory -ErrorAction SilentlyContinue > $null

if ($env:GUI -ne "false") {
  New-Item -Path dist/libs/dialog -ItemType Directory -ErrorAction SilentlyContinue > $null
}

$buildTarget = $env:BUILD_TARGET
$buildTarget = if ($buildTarget) { "/$($buildTarget)" } else {}

# Copy smelt.exe
Copy-Item -Path "./smelt/target$($buildTarget)/$($buildType)/*" -Filter smelt* -Destination "./dist/" -Recurse

# Copy lrt, lgui
Copy-Item -Path "./lrt/target$($buildTarget)/$($buildType)/*" -Filter lrt* -Destination "./dist/" -Recurse
Copy-Item -Path "./lrt/target$($buildTarget)/$($buildType)/*" -Filter lgui* -Destination "./dist/" -Recurse

# Copy linst
Copy-Item -Path "./linst/target$($buildTarget)/$($buildType)/*" -Filter linst* -Destination "./dist/" -Recurse

Copy-Item -Path "./async_waker/target$($buildTarget)/$($buildType)/*" -Filter async_waker* -Destination "./dist/libs/waker/" -Recurse
Copy-Item -Path "./async_waker/target$($buildTarget)/$($buildType)/*" -Filter libasync_waker* -Destination "./dist/libs/waker/" -Recurse

if ($env:GUI -ne "false") {
  Copy-Item -Path "./dialog/target$($buildTarget)/$($buildType)/*" -Filter intl_dialog* -Destination "./dist/libs/dialog/" -Recurse
  Copy-Item -Path "./dialog/target$($buildTarget)/$($buildType)/*" -Filter libintl_dialog* -Destination "./dist/libs/dialog/" -Recurse
}