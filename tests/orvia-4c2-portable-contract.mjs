/**
 * ORVIA 4C.2 source-only native portability safety contract.
 * Does not run Windows, access real E/F disks, certify no OS-level C writes,
 * execute Cargo tests, or approve a Desktop release.
 */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const read = (file) => readFileSync(resolve(root, file), "utf8");
const config = read("apps/soundcraft/src/config.rs");
const main = read("apps/soundcraft/src/main.rs");
const stage = read("scripts/orvia-4c2-stage-windows-qa.ps1");
const wix = read("packaging/windows/soundcraft.wxs");
const native = read("scripts/orvia-4c-native-qa.sh");
const web = read("scripts/cloudflare-pages-build.sh");

let passed = 0;
function check(name, expected) {
  for (const [source, needle] of expected) {
    assert.ok(source.includes(needle), name + ": missing " + JSON.stringify(needle));
  }
  console.log("PASS ORVIA 4C.2: " + name);
  passed++;
}
check("explicit marker and local ORVIA-Data", [
  [config, 'root.join(".orvia-portable")'],
  [config, 'root.join("ORVIA-Data")'],
  [config, 'portable_directory_for(root, &system_drive).map(Some)'],
  [config, 'if path[0].to_ascii_uppercase() == system'],
]);
check("GUI startup rejects invalid portable marker before preference I/O", [
  [main, 'if let Err(why) = config::orvia_portable_config_dir()'],
  [main, '.set_title("ORVIA Portable")'],
  [main, '.set_level(rfd::MessageLevel::Error)'],
]);
check("native Rust data-path safety tests exist", [
  [config, 'fn orvia_portable_keeps_native_app_data_on_non_system_drives()'],
  [config, 'portable_directory_for(std::path::Path::new'],
  [config, 'PathBuf::from('],
  [config, '"C:"'],
  [config, '"E:"'],
]);
check("staging refuses C:, verifies SHA256/PE and avoids overwrite", [
  [stage, "[ValidatePattern("],
  [stage, '$root -notmatch'],
  [stage, '$env:SystemDrive'],
  [stage, 'Destination already exists'],
  [stage, 'Get-FileHash -LiteralPath $exe -Algorithm SHA256'],
  [stage, '$pe.Machine -ne 0x8664 -or $pe.Subsystem -ne 2'],
  [stage, "New-Item -ItemType File -Path (Join-Path $destinationPath '.orvia-portable')"],
]);
check("public release HOLD — no upstream masquerading", [
  [stage, "ORVIA Desktop — INTERNAL QA ONLY"],
  [stage, "NOT FOR PUBLIC RELEASE"],
  [wix, 'Name="SoundCraft"'],
  [wix, 'Manufacturer="Learning Machines LLC"'],
  [native, "cargo test --locked -p soundcraft-engine --test orvia_4c_release_smoke"],
]);
check("Cloudflare checks source, never publishes Windows", [
  [web, "node tests/orvia-4c-desktop-readiness.mjs"],
  [web, "node tests/orvia-4c2-portable-contract.mjs"],
]);
console.log("ORVIA 4C.2: " + passed + "/6 source contracts passed; Windows RC remains HOLD.");
