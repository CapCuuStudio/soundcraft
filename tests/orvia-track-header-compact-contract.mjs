/**
 * ORVIA Track Header compact workstation/Cubase-inspired UX regression contract.
 * Source-only, no browser or physical audio validation implied.
 */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const ui = readFileSync(resolve(root, "crates/ui-egui/src/edit_window.rs"), "utf8");
const model = readFileSync(resolve(root, "crates/model/src/track.rs"), "utf8");
let passed = 0;
function check(label, fragments) {
  for (const part of fragments) assert.ok(ui.includes(part), label + ": missing " + JSON.stringify(part));
  console.log("PASS ORVIA Track Header: " + label);
  passed++;
}
check("workstation-neutral selected and idle track color", [
  "fn compact_track_header_color(", "Color32::from_rgb(37, 42, 47)",
  "Color32::from_rgb(61, 67, 72)", "const STRIP_W: f32 = 5.0",
  "let head_bg = compact_track_header_color(",
]);
check("compact two-row M/S and Record/Monitor/Edit/Automation mapping", [
  "compact_track_header_rows(h)", "HeaderKey::Mute, m.mute",
  "HeaderKey::Solo, m.solo", "HeaderKey::Rec, m.record_arm",
  "HeaderKey::Input, m.input_monitor",
  '("edit_track", id.0)', '("auto_lane", id.0)',
  '"mix.mute"', '"mix.solo"', '"mix.record_arm"', '"mix.input_monitor"',
]);
check("real track menu, selection, name reorder, folder collapse", [
  '"track.move"', '"track.folder_toggle"', '"track.height"',
  "click_track(app, id", "open_rename_track",
  'track_context_menu(app, ui, id)', '("pl", id.0)',
]);
check("playlist and advanced track commands survive collapse", [
  '"track.playlist_new"', '"track.playlist_duplicate"',
  '"track.playlist_delete_unused"', '"track.playlist_select"',
  '"track.view"', '"track.freeze"', '"track.make_inactive"',
  '"mix.automation_mode"', '"track.timebase"', '"track.elastic"',
]);
check("track geometry and source data colors remain unchanged", [
  "pub const HEADER_W: f32 = 136.0", "pub fn track_height(t: &Track)",
  "let lane = lane_rect(track, row, tl)",
  "let (_old_head_bg, lane_bg) = crate::theme::track_backgrounds(",
  "let h = head.height()", "header_meter(ui",
]);
check("real native UI palette and density unit tests exist", [
  "mod compact_track_header_tests",
  "fn normal_track_heights_keep_controls_compact",
  "fn selected_track_uses_grey_instead_of_cyan_fill",
  "TrackHeight::Small.points()", "TrackHeight::Medium.points()",
]);
for (const preset of ["Micro", "Mini", "Small", "Medium", "Large"]) {
  assert.ok(model.includes("TrackHeight::" + preset), "track size missing: " + preset);
}
console.log("ORVIA Track Header source contracts: " + passed + "/6 passed, native Rust and browser QA pending.");
