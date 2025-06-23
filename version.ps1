$ver = Get-Content ./.version

$millis = [System.DateTimeOffset]::Now.ToUnixTimeMilliseconds()

"$millis" > .build
"$ver+$millis" > .version
"v$ver+$millis" > .tag

"Version: $ver-$millis (tag v$millis)"