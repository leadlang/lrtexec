./build.ps1

# Packaging Steps, if any
Set-Location dist

Set-Location ..

Compress-Archive -Path dist/* -DestinationPath $($env:BUILD_TARGET).zip