/**
 * ORVIA 4B.4 non-browser responsive/source contract.
 * Only built-in Node.js modules; prevents regressions to panel scrolling,
 * commands and prior layout boundaries before expensive Rust WASM compile.
 * This test does not claim pixel-level Chrome or audio/MIDI runtime QA.
 */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const file = (p) => readFileSync(resolve(root, p), "utf8");
const panels = file("crates/ui-egui/src/panels.rs");
const rack = file("crates/ui-egui/src/panels/orvia_rack.rs");
const dock = file("crates/ui-egui/src/docking.rs");
const toolbar = file("crates/ui-egui/src/toolbar.rs");
const edit = file("crates/ui-egui/src/edit_window.rs");
const midi = file("crates/ui-egui/src/midi_editor.rs");
const widgets = file("crates/ui-egui/src/widgets.rs");
const icons = file("crates/ui-egui/src/icons.rs");

let passed = 0;
function check(name, fn) {
  try {
    fn();
    passed++;
    process.stdout.write(`PASS ORVIA 4B.4: ${name}\n`);
  } catch (error) {
    process.stderr.write(`FAIL ORVIA 4B.4: ${name}: ${error.message}\n`);
    process.exitCode = 1;
  }
}
function must(s, fragments) {
  for (const f of fragments) assert.ok(s.includes(f), `missing ${JSON.stringify(f)}`);
}
check("MixConsole keeps both-axis scrolling for 420x180 dock", () => {
  must(panels, [
    'egui::ScrollArea::both().id_salt("orvia_lower_mix_scroll")',
    'ScrollBarVisibility::VisibleWhenNeeded',
    'orvia_mix_ids(app.engine.session(), selected_only)',
    '"mix.mute"', '"mix.solo"', '"mix.volume"', '"mix.pan"',
    '"mix.send_level"', 'crate::mix_window::route_popup',
  ]);
  must(dock, ["Panel::MixConsole => (egui::vec2(420.0, 180.0)"]);
});
check("Studio Rack scrolls complete content, not just plugin catalog", () => {
  must(panels, [
    'pub fn orvia_browser(app: &mut SoundApp, ui: &mut Ui)',
    'egui::ScrollArea::vertical()',
    '.id_salt("orvia-4b4-rack-scroll")',
    'orvia_rack::show(app, ui)',
  ]);
  assert.ok(!rack.includes('ScrollArea::vertical().id_salt("orvia_3c_rack_catalog")'));
  must(rack, [
    '"Nhạc cụ"', '"Hiệu ứng"', '"TRACK ĐÍCH"',
    '"mix.instrument"', '"mix.insert"', '"mix.insert_bypass"', '"mix.insert_remove"',
  ]);
  must(dock, ["Panel::Browser => (egui::vec2(200.0, 160.0)"]);
});
check("Inspector stays scrollable, resizable, and connected to commands", () => {
  must(panels, [
    'pub fn orvia_inspector(app: &mut SoundApp, ui: &mut Ui)',
    '.id_salt("orvia_inspector_scroll")', 'app.dialogs.open_rename_track',
  ]);
  must(dock, [
    "SplitSize::FixedFirst(270.0)",
    "Panel::Inspector => (egui::vec2(190.0, 160.0), 410.0)",
  ]);
});
check("Key Editor maintains MIDI geometry, commands and narrow-body scroll", () => {
  must(midi, [
    "const ORVIA_KEY_EDITOR_HEADER_H: f32 = 56.0",
    '"event.quantize"', '"event.change_velocity"', '"event.change_duration"',
    '"midi.note_add"', '"midi.note_delete"', '"midi.notes_move"',
    "controller_lane::show(",
  ]);
  must(dock, [
    "minimum_body(ui, egui::vec2(420.0, 260.0)",
    "egui::ScrollArea::both().id_salt(\"minimum-body\")",
  ]);
});
check("Toolbar retains 1366/1440/1738 responsive regimes", () => {
  must(toolbar, [
    "pub const HEIGHT: f32 = 140.0",
    "pub const WIDE_HEIGHT: f32 = 78.0",
    "const SINGLE_ROW_MIN_WIDTH: f32 = 1_550.0",
    "if width >= SINGLE_ROW_MIN_WIDTH { WIDE_HEIGHT } else { HEIGHT }",
    '.id_salt("orvia_toolbar_transport_row")',
    '.id_salt("orvia_toolbar_edit_row")',
    '"transport.play"', '"transport.stop"', '"transport.record"',
  ]);
  must(edit, ["toolbar::height_for_width(ui.available_width())"]);
});
check("Timeline, Track state and clip commands remain present", () => {
  must(edit, [
    '"mixConsole"', '"midi"',
    '"ui.dock"', '"ui.dock.reset"',
    "ORVIA_LOWER_TAB_HEIGHT",
    '"MIXCONSOLE"', '"KEY EDITOR"',
  ]);
  must(dock, ["Panel::Canvas =>", "Panel::MixConsole =>", "Panel::Midi =>"]);
});
check("4B.3 brand and state visuals survive responsive fixes", () => {
  must(icons, ['"orvia_mark"']);
  must(widgets, ["orvia_icon_palette", "focus_stroke"]);
  must(toolbar, ["orvia_transport_skin", "orvia_transport_face"]);
});
console.log(`ORVIA 4B.4 source contract: ${passed}/7 passed`);
if (process.exitCode) process.exit(process.exitCode);
