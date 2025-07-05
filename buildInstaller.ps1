$token = $env:TOKEN
$token = ConvertTo-SecureString $token -AsPlainText -Force

Invoke-WebRequest -Uri "$env:URL" -OutFile "$env:TARGET.zip" -Authentication Bearer -Token $token

Expand-Archive -Path "$env:TARGET.zip" -DestinationPath .\installers\windows\build\ -Force -Verbose

Set-Location .\installers\windows

# Implicitly requires variable $env:ARCH
.\build.ps1

Set-Location ..\..

Copy-Item -Path ".\installers\windows\bin\$env:ARCH\Release\installer.msi" -Destination ".\lrtexec_$env:TARGET`_installer.msi"