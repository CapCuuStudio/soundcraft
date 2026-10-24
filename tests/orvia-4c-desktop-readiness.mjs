/**
 * ORVIA 4C Desktop preflight, no extra dependencies or Windows runner.
 *
 * PASS: source contracts for release preparation exist.
 * HOLD: branded installer/hardware tests remain unverified. A passing preflight
 * MUST NOT be interpreted as a native release or test success.
 */
import assert from "node:assert/strict";
import { readFileSync, existsSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const read = (path) => readFileSync(resolve(root, path), "utf8");
const present = (path) => assert.ok(existsSync(resolve(root, path)), `missing ${path}`);
const must = (source, value) => assert.ok(source.includes(value), `missing: ${value}`);

const tests = [
  "crates/engine/tests/orvia_4c_release_smoke.rs",
  "crates/engine/tests/recording.rs",
  "crates/engine/tests/bounce.rs",
  "crates/engine/tests/session_transport.rs",
  "crates/engine/tests/midi_offset.rs",
  "crates/midi/tests/smf.rs",
  "crates/ui-egui/tests/docking_workflows.rs",
  "crates/ui-egui/tests/save_as_replace.rs",
  "crates/ui-egui/tests/midi_quantize.rs",
  "crates/ui-egui/tests/sampler_window.rs",
  "crates/ui-egui/tests/plugin_window.rs",
];

for (const file of tests) present(file);
const smoke = read(tests[0]);
for (const id of [
  '"track.new"', '"midi.new_clip"', '"midi.note_add"', '"midi.cc_set"',
  '"midi.notes"', '"mix.insert"', '"session.save_as"', '"session.open"',
  '"session.new"', '"session.close"', '"session.revert"',
  'soundcraft_mix::render_range', 'fn orvia_4c_midi_note_pedal_synth_and_mix_are_real',
  'fn orvia_4c_session_save_and_reopen_preserves_midi_pedal_and_audio',
  'fn orvia_4c_recording_never_replaces_a_live_session',
]) must(smoke, id);
console.log(`PASS 4C source preflight: ${tests.length} native test files and 3 cross-functional Rust tests present`);

const manifest = read("apps/soundcraft/Cargo.toml");
const wix = read("packaging/windows/soundcraft.wxs");
const powershell = read("packaging/windows/package.ps1");
const release = read(".github/workflows/release.yml");
const config = read("apps/soundcraft/src/config.rs");
must(manifest, 'name = "soundcraft"');
must(wix, 'Name="SoundCraft"');
must(wix, 'Manufacturer="Learning Machines LLC"');
must(powershell, 'cargo build --release --locked -p soundcraft');
must(powershell, "Get-PeHeader");
must(release, "branches: [release]");
must(release, "workflow_dispatch:");
must(config, "SOUNDCRAFT_CONFIG_DIR");
present("scripts/orvia-4c-native-qa.sh");

const hold = [];
if (!/Name="ORVIA"/.test(wix)) hold.push("Windows MSI package still identifies as SoundCraft");
if (/Manufacturer="Learning Machines LLC"/.test(wix)) hold.push("installer manufacturer is original upstream, not approved ORVIA publisher");
if (/soundcraft\.exe/.test(powershell)) hold.push("Windows binary/portable assets retain SoundCraft naming");
if (/assets[\\/]app-icon[\\/]soundcraft\.ico/.test(powershell)) hold.push("desktop icon has not been verified as original ORVIA artwork");
hold.push("native Cargo tests and Windows x64 executable have not run in this Cloudflare Web build");
hold.push("audio device/ASIO latency, hardware MIDI input, Kontakt 8/VST3 hosting, Save/Open and real desktop playback await authorized Windows UAT");
hold.push("code-signing identity, installer registration and security scan require release-approval evidence");

console.log("PASS 4C packaging source inspection: native MSI build and PE architecture checks exist");
console.log("HOLD ORVIA Desktop release: " + hold.join("; "));
console.log("Result: SOURCE_PREPARED, DESKTOP_RC_NOT_APPROVED");
