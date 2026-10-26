# ORVIA 4C.4 — Hosted Windows CI PE/SHA provenance gate.
# Reads already built files ONLY. No install, app launch, signing, network or registry.
# This is not a Kontakt, ASIO, physical soundcard or user-laptop runtime test.
param(
    [Parameter(Mandatory = $true)][string] $GuiBinaryPath,
    [Parameter(Mandatory = $true)][string] $CliBinaryPath,
    [Parameter(Mandatory = $true)][string] $ExpectedCommit,
    [string] $ReportFile
)
$ErrorActionPreference = 'Stop'
if ($ExpectedCommit -notmatch '^[a-fA-F0-9]{40}$') {
    throw 'ExpectedCommit must be an exact 40-character Git commit SHA.'
}

function Read-PeHeader([string] $BinaryPath, [int] $ExpectedSubsystem) {
    $file = (Resolve-Path -LiteralPath $BinaryPath -ErrorAction Stop).ProviderPath
    if ((Get-Item -LiteralPath $file).PSIsContainer) { throw 'Expected binary file, got folder.' }
    $stream = [System.IO.File]::Open($file, [IO.FileMode]::Open, [IO.FileAccess]::Read, [IO.FileShare]::Read)
    try {
        $r = [System.IO.BinaryReader]::new($stream)
        try {
            if ($stream.Length -lt 256 -or $r.ReadUInt16() -ne 0x5A4D) {
                throw 'Not a DOS/PE file.'
            }
            $stream.Position = 0x3C
            $offset = $r.ReadInt32()
            if ($offset -lt 64 -or ($offset + 0x60) -gt $stream.Length) {
                throw 'Invalid PE header offset.'
            }
            $stream.Position = $offset
            if ($r.ReadUInt32() -ne 0x00004550) { throw 'Missing PE signature.' }
            $machine = $r.ReadUInt16()
            $stream.Position = $offset + 0x18
            $magic = $r.ReadUInt16()
            $stream.Position = $offset + 0x5C
            $subsystem = $r.ReadUInt16()
            if ($machine -ne 0x8664 -or $magic -ne 0x20B -or $subsystem -ne $ExpectedSubsystem) {
                throw "Wrong PE machine/format/subsystem: machine=$machine magic=$magic subsystem=$subsystem"
            }
            $sha = (Get-FileHash -LiteralPath $file -Algorithm SHA256).Hash.ToLowerInvariant()
            return @{
                sha256 = $sha
                bytes = $stream.Length
                machine = 'x86_64'
                subsystem = $(if ($ExpectedSubsystem -eq 2) { 'GUI' } else { 'CONSOLE' })
            }
        } finally { $r.Dispose() }
    } finally { $stream.Dispose() }
}

$gui = Read-PeHeader -BinaryPath $GuiBinaryPath -ExpectedSubsystem 2
$cli = Read-PeHeader -BinaryPath $CliBinaryPath -ExpectedSubsystem 3
$report = [ordered]@{
    product = 'ORVIA/SoundCraft source — INTERNAL NATIVE QA, NOT FOR RELEASE'
    stage = '4C.4'
    git_commit_sha = $ExpectedCommit.ToLowerInvariant()
    gui = $gui
    cli = $cli
    provenance_result = 'PASS: x64 PE headers and SHA256 generated on hosted Windows'
    runtime_result = 'HOLD: the GUI executable was not launched'
    asio_result = 'HOLD: optional SDK-enabled ASIO build/physical driver not tested'
    kontakt_vst3_nki = 'HOLD: licensed third-party Kontakt 8 not installed in CI'
    external_drive_and_c_writes = 'HOLD: owner E:/F: portable runtime not tested'
    branding = 'HOLD: upstream SoundCraft icon, publisher and binary identity'
    public_release = 'HOLD'
}
$text = $report | ConvertTo-Json -Depth 5
if ($ReportFile) {
    if (Test-Path -LiteralPath $ReportFile) { throw 'Refusing to overwrite an existing provenance report.' }
    $parent = Split-Path -Parent ([IO.Path]::GetFullPath($ReportFile))
    if (-not (Test-Path -LiteralPath $parent -PathType Container)) { throw 'Report parent does not exist.' }
    [IO.File]::WriteAllText([IO.Path]::GetFullPath($ReportFile), $text, [Text.UTF8Encoding]::new($false))
}
Write-Output $text
Write-Output 'Windows binary provenance PASS only. Desktop ASIO/Kontakt/brand/release remain HOLD.'
