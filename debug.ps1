./build.ps1
Set-Location dist

New-Item -Path test -ItemType Directory > $null
Copy-Item -Path ../test/* -Destination ./test/ -Recurse > $null

"'Reloading'; Set-Location ..; ./debug.ps1" > debug.ps1