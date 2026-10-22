# ORVIA Web — Stage 3B: Piano Roll / Velocity / MIDI CC

**Status:** implementation committed on isolated branch; Cloudflare WASM build and Chrome UAT required before acceptance.  
**Base:** Stage 3A at `cd61200cc14d2525a1098412a37d296f659591a5`.  
**Branch:** `feature/orvia-3b`.

## Scope

- Reuse the existing real Piano Roll, note add/drag/resize/quantize/delete/Undo workflow. Do not create a fake web-only piano.
- Add the lower controller lane selector: Velocity, CC1 Modulation, CC7 Volume, CC10 Pan, CC11 Expression and CC64 Sustain.
- The actual `soundcraft_midi::Sequence.ctrls` owns CC events; each controller value is on MIDI channel 0 (displayed as channel 1), at a quantized tick. Click or drag to set; secondary-click near a point to remove.
- Register `midi.cc_set` and `midi.cc_delete` engine commands, replacing an existing same-tick/same-CC/same-channel event. Reject out-of-range MIDI CC, tick, channel or value and missing or locked clips. Undo/Redo flows through engine history.
- UI selection resets on changing MIDI clips. Controller lane selection is local ephemeral UI data and does not affect session serialization.
- No additional samples, plugins, billing services, production changes or Windows user CMD.

## Gates

1. GitHub source commit matches the Cloudflare Pages **Deploy successful** bot message and immutable preview URL for that SHA; do not infer deployment from GitHub.
2. Rust `soundcraft-engine` tests including `orvia3b_controller_edit_replace_delete_undo_bundle_and_locks`; UI tests including `lane_names_include_pedal_and_velocity` (no Cargo available in sandbox: **unverified**).
3. Browser: select Instrument/MIDI track, add a clip, open Lower Zone MIDI Editor, add 3 notes, move/stretch, modify velocity of each independently, undo and redo.
4. Switch to each controller: CC1/7/10/11/64; click at separate ticks to set value, drag to change, right-click to delete. Existing controller types and notes must remain unchanged.
5. CC64 value 127 and 0 must persist through Save/Open .scraftz and export/import MIDI, without changing note timing or velocity. **Audible sustain is not guaranteed** unless the selected instrument implements pedal.
6. Locked MIDI clip: editing controller and notes must not modify its data. Switch clips: stale note indices must not apply to a different clip.
7. 1366×768 browser: header selector, piano keys, notes, and controller lane remain usable. Verify audio Play/Stop, Mixer 3A, Save/Open, Undo and import/export unchanged.

**Do not merge main or production without explicit approval.** Preserve Stage 3A and Stage 2C immutable baselines. No self-triggered GitHub Actions or paid services.
