/**
 * ORVIA 4C.3 - dependency-free SOURCE contract, not Windows runtime QA.
 * A passing Cloudflare/Web run never certifies native Windows, ASIO or Kontakt.
 */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";
const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const read = (path) => readFileSync(resolve(root, path), "utf8");
const stage = read("scripts/orvia-4c2-stage-windows-qa.ps1");
const inspect = read("scripts/orvia-4c3-inspect-windows-qa.ps1");
const matrix = read("docs/ORVIA-STAGE4C3-WINDOWS-NATIVE-UAT.md");
const web = read("scripts/cloudflare-pages-build.sh");

let passed = 0;
function check(title, source, needles) {
  for (const needle of needles) {
    assert.ok(source.includes(needle), title + ": missing " + JSON.stringify(needle));
  }
  passed++;
  console.log("PASS 4C.3 source: " + title);
}
check("stage rejects drive-relative and indirect C: via reparse ancestors", stage, [
  "$Destination -notmatch", "Assert-NoReparseAncestors", "ReparsePoint",
  "Assert-NoReparseAncestors $destinationPath", "Destination already exists",
  "Get-FileHash -LiteralPath $exe -Algorithm SHA256", "New-Item -ItemType Directory"
]);
assert.ok(stage.indexOf("Assert-NoReparseAncestors $destinationPath") <
          stage.indexOf("New-Item -ItemType Directory -Path $destinationPath"),
          "Staging must validate junctions BEFORE creating destination");
check("evidence never launches an unverified binary", inspect, [
  "Assert-LocalSafePath", "ReparsePoint", "Get-FileHash -LiteralPath $binary -Algorithm SHA256",
  "ConvertFrom-Json", "ORVIA-QA-ONLY.json", "ReadUInt32()", "0x8664",
  "0x00004550", "$manifest.binary_sha256", "'desktop_release'"]);
check("plugin and audio are explicit HOLD until hands-on QA", inspect, [
  "native_cargo_tests = 'HOLD", "audio_asio = 'HOLD",
  "vst3_kontakt8_nki = 'HOLD", "system_drive_write_audit = 'HOLD",
  "desktop_release = 'HOLD'", "branding_signing = 'HOLD",
  "No network access, plugins launched, installation, registry edits or app execution"
]);
check("no evidence overwrites an existing file or uses C:", inspect, [
  "if (Test-Path -LiteralPath $output)", "Only absolute E:", 
  "Evidence parent directory must exist on E: or F:", "Set-Content -LiteralPath $output",
  "No network access"
]);
check("native UAT matrix with real audio and Kontakt is documented", matrix, [
  "ASIO", "Kontakt 8", "NKI", "4–6", "G1", "G10", "HOLD",
  "Save/Open", "E:/F:", "Cloudflare"
]);
check("Cloudflare runs only this source check", web, [
  "node tests/orvia-4c3-windows-qa-contract.mjs",
  "trunk build --release"
]);
console.log("ORVIA 4C.3: " + passed + "/6 source contracts PASS; Windows native, hardware, release = HOLD.");
