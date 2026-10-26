# ORVIA 4C.3 - READ-ONLY inspection of an INTERNAL QA Windows x64 portable stage.
# Intended for approved QA staff only. NEVER auto-run on the owner's laptop.
# This is NOT a Windows audio test, ASIO test, Kontakt test or release certificate.
# No network access, plugins launched, installation, registry edits or app execution.
param(
    [Parameter(Mandatory = $true)][string] $StageDirectory,
    [Parameter(Mandatory = $true)][ValidatePattern('^[a-fA-F0-9]{64}$')][string] $ExpectedSha256,
    [string] $EvidenceOutput
)
$ErrorActionPreference = 'Stop'

function Assert-LocalSafePath([string] $Value, [bool] $MustExist) {
    if ($Value -notmatch '^[EeFf]:[\\/]') {
        throw 'Only absolute E:\ or F:\ local paths are allowed for ORVIA QA.'
    }
    $full = [IO.Path]::GetFullPath($Value)
    $root = [IO.Path]::GetPathRoot($full)
    if ($root -notmatch '^[EeFf]:[\\/]$') {
        throw 'Network, relative and system-drive paths are not eligible.'
    }
    $system = if ($env:SystemDrive) { $env:SystemDrive.TrimEnd('\') } else { 'C:' }
    if ($root.Substring(0, 2).Equals($system, [StringComparison]::OrdinalIgnoreCase)) {
        throw 'The path resolves to the Windows system drive.'
    }
    $cursor = if ($MustExist) { $full } else { Split-Path -Parent $full }
    while (-not [string]::IsNullOrEmpty($cursor)) {
        if (Test-Path -LiteralPath $cursor) {
            $entry = Get-Item -LiteralPath $cursor -Force
            if (($entry.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0) {
                throw 'Junction/symlink in QA path; cannot certify safe local staging.'
            }
        }
        $parent = Split-Path -Parent $cursor
        if ($parent -eq $cursor) { break }
        $cursor = $parent
    }
    if ($MustExist -and -not (Test-Path -LiteralPath $full -PathType Container)) {
        throw 'Internal QA stage folder does not exist.'
    }
    return $full
}

$stage = Assert-LocalSafePath $StageDirectory $true
$binary = Join-Path $stage 'soundcraft.exe'
$marker = Join-Path $stage '.orvia-portable'
$manifestPath = Join-Path $stage 'ORVIA-QA-ONLY.json'
foreach ($file in @($binary, $marker, $manifestPath)) {
    if (-not (Test-Path -LiteralPath $file -PathType Leaf)) {
        throw 'Required portable stage file is missing.'
    }
    if (((Get-Item -LiteralPath $file -Force).Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0) {
        throw 'QA stage contains a reparse-point file.'
    }
}
if ((Get-Item -LiteralPath $manifestPath).Length -gt 32768) {
    throw 'Unreasonably large stage manifest; refusing untrusted content.'
}
$manifest = Get-Content -LiteralPath $manifestPath -Raw | ConvertFrom-Json
if ($manifest.product -ne 'ORVIA Desktop — INTERNAL QA ONLY' -or
    $manifest.stage -ne '4C.2' -or
    $manifest.launch -ne 'soundcraft.exe' -or
    $manifest.portable_marker -ne '.orvia-portable' -or
    $manifest.application_data_relative -ne 'ORVIA-Data') {
    throw 'Not a recognized internal 4C.2 portable QA stage.'
}
$hash = (Get-FileHash -LiteralPath $binary -Algorithm SHA256).Hash.ToLowerInvariant()
if ($hash -ne $ExpectedSha256.ToLowerInvariant() -or
    $hash -ne ([string]$manifest.binary_sha256).ToLowerInvariant()) {
    throw 'Binary SHA256 mismatch. Evidence not accepted.'
}
# Read PE header only: do not launch or load the executable.
$stream = [IO.File]::OpenRead($binary)
try {
    $reader = [IO.BinaryReader]::new($stream)
    try {
        if ($stream.Length -lt 256 -or $reader.ReadUInt16() -ne 0x5A4D) {
            throw 'Not a Windows PE executable.'
        }
        $stream.Seek(0x3C, [IO.SeekOrigin]::Begin) | Out-Null
        $offset = $reader.ReadInt32()
        if ($offset -lt 64 -or ($offset + 0x60) -gt $stream.Length) {
            throw 'Malformed PE offset.'
        }
        $stream.Seek($offset, [IO.SeekOrigin]::Begin) | Out-Null
        if ($reader.ReadUInt32() -ne 0x00004550) {
            throw 'Invalid PE signature.'
        }
        $machine = $reader.ReadUInt16()
        $stream.Seek($offset + 0x5C, [IO.SeekOrigin]::Begin) | Out-Null
        $subsystem = $reader.ReadUInt16()
        if ($machine -ne 0x8664 -or $subsystem -ne 2) {
            throw 'Expected Windows x64 GUI executable; not approved.'
        }
    } finally { $reader.Dispose() }
} finally { $stream.Dispose() }

# Never put usernames, drive paths, plugin inventory, raw project data or tokens in evidence.
$evidence = [ordered]@{
    product = 'ORVIA Desktop - INTERNAL QA ONLY'
    gate = '4C.3'
    verified_utc = (Get-Date).ToUniversalTime().ToString('o')
    binary_sha256 = $hash
    portable_marker_present = $true
    stage_manifest_verified = $true
    pe_machine = 'x64'
    pe_subsystem = 'Windows GUI'
    path_guard = 'E/F; system drive, known reparse paths and UNC refused'
    native_cargo_tests = 'HOLD - no test log supplied'
    portable_runtime = 'HOLD - not launched'
    system_drive_write_audit = 'HOLD - not monitored'
    audio_asio = 'HOLD - owner hardware test required'
    vst3_kontakt8_nki = 'HOLD - owner licensed installation required'
    save_open_bounce_soak = 'HOLD - GUI test required'
    branding_signing = 'HOLD - upstream binary and metadata'
    desktop_release = 'HOLD'
}
if ($EvidenceOutput) {
    $output = Assert-LocalSafePath $EvidenceOutput $false
    if (Test-Path -LiteralPath $output) {
        throw 'Evidence output exists; refusing overwrite.'
    }
    $parent = Split-Path -Parent $output
    if (-not (Test-Path -LiteralPath $parent -PathType Container)) {
        throw 'Evidence parent directory must exist on E: or F:.'
    }
    $evidence | ConvertTo-Json -Depth 4 |
        Set-Content -LiteralPath $output -Encoding UTF8
}
$evidence | ConvertTo-Json -Depth 4
Write-Output 'Static provenance PASS only. ORVIA Desktop release remains HOLD.'