# ORVIA 4C.2 — internal QA staging ONLY; not an MSI or consumer release.
# Run exclusively on an approved Windows QA machine with an already-built,
# SHA256-verified x64 native executable. Never run automatically on the owner PC.
# No install, registry edits, app execution, signing or network access.
param(
    [Parameter(Mandatory = $true)][string] $BinaryPath,
    [Parameter(Mandatory = $true)][string] $Destination,
    [Parameter(Mandatory = $true)][ValidatePattern('^[a-fA-F0-9]{64}$')][string] $ExpectedSha256,
    [string] $CliBinaryPath
)
$ErrorActionPreference = 'Stop'

function Get-PeSignature([string] $Path) {
    $data = [IO.File]::ReadAllBytes($Path)
    if ($data.Length -lt 256 -or $data[0] -ne 0x4D -or $data[1] -ne 0x5A) {
        throw 'Native QA app is not a Windows PE executable.'
    }
    $offset = [BitConverter]::ToInt32($data, 0x3C)
    if ($offset -lt 64 -or $offset + 0x60 -gt $data.Length) {
        throw 'Malformed Windows PE header.'
    }
    if ([BitConverter]::ToUInt32($data, $offset) -ne 0x00004550) {
        throw 'Invalid Windows PE signature.'
    }
    return @{
        Machine   = [BitConverter]::ToUInt16($data, $offset + 4)
        Subsystem = [BitConverter]::ToUInt16($data, $offset + 0x5C)
    }
}

$exe = (Resolve-Path -LiteralPath $BinaryPath).ProviderPath
$destinationPath = [IO.Path]::GetFullPath($Destination)
$root = [IO.Path]::GetPathRoot($destinationPath)
# Owner constraint: only E:/F:. Reject system drive, UNC, relative, network.
if ($root -notmatch '^[EeFf]:[\\/]$') {
    throw 'ORVIA Portable QA must be staged to a local E: or F: drive. C: and network drives are refused.'
}
$systemDrive = if ($env:SystemDrive) { $env:SystemDrive.TrimEnd('\\') } else { 'C:' }
if ($root.Substring(0, 2).Equals($systemDrive, [StringComparison]::OrdinalIgnoreCase)) {
    throw 'The selected destination is the Windows system drive. Staging blocked.'
}
if (Test-Path -LiteralPath $destinationPath) {
    throw 'Destination already exists; refusing to overwrite any project or prior portable folder.'
}
if ((Split-Path -Leaf $exe) -ne 'soundcraft.exe') {
    throw 'Source executable name must match the verified internal engine binary soundcraft.exe.'
}

$actualSha256 = (Get-FileHash -LiteralPath $exe -Algorithm SHA256).Hash
if ($actualSha256 -ine $ExpectedSha256) {
    throw 'Executable SHA256 differs from the approved build. No staging performed.'
}
$pe = Get-PeSignature $exe
if ($pe.Machine -ne 0x8664 -or $pe.Subsystem -ne 2) {
    throw 'Expected Windows x64 GUI subsystem PE; source executable is not eligible.'
}

$cli = $null
if ($CliBinaryPath) {
    $cli = (Resolve-Path -LiteralPath $CliBinaryPath).ProviderPath
    $cliHeader = Get-PeSignature $cli
    if ($cliHeader.Machine -ne 0x8664 -or $cliHeader.Subsystem -ne 3) {
        throw 'CLI executable must be x64 console subsystem.'
    }
}

# No changes are made before integrity, disk-safety and non-overwrite checks.
New-Item -ItemType Directory -Path $destinationPath -Force | Out-Null
Copy-Item -LiteralPath $exe -Destination (Join-Path $destinationPath 'soundcraft.exe')
if ($cli) {
    Copy-Item -LiteralPath $cli -Destination (Join-Path $destinationPath 'soundcraft-cli.exe')
}
New-Item -ItemType File -Path (Join-Path $destinationPath '.orvia-portable') | Out-Null

$sourceRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).ProviderPath
foreach ($file in 'LICENSE', 'LICENSE-MIT', 'LICENSE-APACHE', 'NOTICE', 'ATTRIBUTION.md') {
    $sourceFile = Join-Path $sourceRoot $file
    if (Test-Path -LiteralPath $sourceFile) {
        Copy-Item -LiteralPath $sourceFile -Destination (Join-Path $destinationPath $file)
    }
}
$manifest = @{
    product = 'ORVIA Desktop — INTERNAL QA ONLY'
    stage = '4C.2'
    launch = 'soundcraft.exe'
    disclaimer = 'Original binary/icon/metadata still need ORVIA branding and Windows native hardware QA. NOT FOR PUBLIC RELEASE.'
    binary_sha256 = $actualSha256
    portable_marker = '.orvia-portable'
    application_data_relative = 'ORVIA-Data'
    original_engine = 'SoundCraft'
}
$manifest | ConvertTo-Json -Depth 3 | Set-Content -LiteralPath (Join-Path $destinationPath 'ORVIA-QA-ONLY.json') -Encoding UTF8
Write-Output 'ORVIA QA staging prepared. This does not certify installed plugins, audio hardware, or Windows cache writes.'
Write-Output 'Do NOT distribute this unbranded, unverified internal binary as an ORVIA release.'
