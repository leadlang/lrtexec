$buildType = $args.Get(0)

$typeof = $args.Get(1)

$special = ""

if ($args.Length -gt 2) {
  $special = $args.Get(2)
}

if ($buildType -eq "debug") {
  $buildType = $null
}
else {
  $buildType = "--$($buildType)"
}

$cross = $False
if ($env:USE_CROSS -eq "true") {
  $cross = $True
}

$buildTarget = $env:BUILD_TARGET
$targetFlag = if ($buildTarget) { "--target" } else {}

switch ($special) {
  "nogui" {
    if ($cross) {
      "Using Cross"
      cross +nightly $($buildType) $($typeof) $($targetFlag) $($buildTarget) --features no_gui
    }
    else {
      rustup run nightly cargo build $($buildType) $($typeof) $($targetFlag) $($buildTarget) --features no_gui
    }
  }
  default {
    if ($cross) {
      "Using Cross"
      cross +nightly $($buildType) $($typeof) $($targetFlag) $($buildTarget)
    }
    else {
      rustup run nightly cargo build $($buildType) $($typeof) $($targetFlag) $($buildTarget)
    }
  }
}