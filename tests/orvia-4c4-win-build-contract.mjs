/**
 * ORVIA 4C.4 — source-only safety contract.
 * Running Node checks does NOT build native Windows or verify ASIO/Kontakt.
 */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";
const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const read = file => readFileSync(resolve(root, file), "utf8");
const workflow = read(".github/workflows/orvia-4c4-windows-native.yml");
const pe = read("scripts/orvia-4c4-pe-provenance.ps1");
const docs = read("docs/ORVIA-STAGE4C4-WINDOWS-BUILD-QA.md");
let passed = 0;
function check(label, actual, expected) {
  for (const token of expected) {
    assert.ok(actual.includes(token), label + ": missing " + JSON.stringify(token));
  }
  ++passed;
  console.log("PASS 4C.4 source contract: " + label);
}
check("branch-isolated non-manual Windows x64 runner", workflow, [
  "on:", "push:", "feature/orvia-4c4-windows-native-build",
  "windows-2022", "contents: read", "timeout-minutes:", "cancel-in-progress: true"
]);
assert.ok(!/^\s*workflow_dispatch:/m.test(workflow), "Do not manually dispatch Windows QA workflow");
assert.ok(!/^\s*pull_request_target:/m.test(workflow), "No privileged pull_request_target");
assert.ok(!/upload-artifact|create-release|wix build|packaging\/windows\/package\.ps1/i.test(workflow),
  "Internal QA should not publish unsigned/upstream-branded binary");
check("actual native Rust tests and executable compilation", workflow, [
  "cargo test --locked --release -p soundcraft-engine --test orvia_4c_release_smoke",
  "cargo test --locked --release -p soundcraft-midi --test smf",
  "cargo test --locked --release -p soundcraft --bin soundcraft config_tests",
  "cargo build --locked --release -p soundcraft -p soundcraft-cli",
  "scripts/orvia-4c4-pe-provenance.ps1"
]);
check("Windows PE/SHA checks without app execution", pe, [
  "Read-PeHeader", "Get-FileHash -LiteralPath $file -Algorithm SHA256",
  "0x8664", "0x20B", "0x00004550", "ExpectedSubsystem 2", "ExpectedSubsystem 3"
]);
check("provenance distinguishes a native binary from real Kontakt/ASIO QA", pe, [
  "runtime_result = 'HOLD", "asio_result = 'HOLD",
  "kontakt_vst3_nki = 'HOLD", "branding = 'HOLD",
  "public_release = 'HOLD'"
]);
check("native QA and user-storage gates are explicitly separate", docs, [
  "Windows x64", "Kontakt 8", "ASIO", "E:/F:", "HOLD", "4–6-minute",
  "No Kontakt installation on shared CI"
]);
console.log("ORVIA 4C.4: " + passed + "/5 static source checks PASS; native Windows CI required.");
