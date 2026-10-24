//! ORVIA 4C.1: run core Desktop release gates with the real Engine, synth,
//! mix renderer and on-disk session format. No audio hardware is needed.
//! These tests must be executed by native Cargo before a Windows RC is approved.

use serde_json::json;
use soundcraft_engine::Engine;
use soundcraft_model::ClipId;
use soundcraft_time::Range;

fn instrument_project() -> (Engine, u64, u64) {
    let mut engine = Engine::default();
    let result = engine
        .execute("track.new", &json!({
            "kind": "instrument",
            "name": "ORVIA Keys",
            "instrument": "subtractive_synth"
        }))
        .expect("a real instrument track must be creatable");
    let track = result["tracks"][0].as_u64().expect("track ID");
    let result = engine
        .execute("midi.new_clip", &json!({
            "track": track, "start": 0, "length": 96_000
        }))
        .expect("a MIDI clip must be creatable");
    let clip = result["clip"].as_u64().expect("MIDI clip ID");
    engine
        .execute("midi.note_add", &json!({
            "clip": clip, "pitch": 60, "start_ticks": 0,
            "length_ticks": 960, "velocity": 91
        }))
        .expect("the Key Editor's note command must succeed");
    engine
        .execute("midi.cc_set", &json!({
            "clip": clip, "cc": 64, "tick": 0, "value": 127
        }))
        .expect("pedal CC64 must be stored");
    (engine, track, clip)
}

fn sample_peak(engine: &Engine) -> f32 {
    let audio = soundcraft_mix::render_range(engine.session(), Range::new(0, 48_000), 512);
    assert!(!audio.is_empty(), "the mixer must render an output");
    assert!(audio.iter().flatten().all(|sample| sample.is_finite()), "audio must remain finite");
    audio.iter().flatten().map(|sample| sample.abs()).fold(0.0_f32, f32::max)
}

#[test]
fn orvia_4c_midi_note_pedal_synth_and_mix_are_real() {
    let (mut engine, track, clip) = instrument_project();
    let notes = engine.execute("midi.notes", &json!({"clip": clip})).unwrap();
    assert_eq!(notes["notes"][0]["pitch"], 60);
    assert_eq!(notes["notes"][0]["velocity"], 91);
    let content = &engine.session().find_clip(ClipId(clip)).unwrap().1.content;
    let content_before_insert = content.clone();
    let peak = sample_peak(&engine);
    assert!(peak > 0.000001, "a real MIDI instrument should produce nonzero PCM");

    // Route a real built-in processor through the same mixer used by playback/bounce.
    let inserted = engine.execute("mix.insert", &json!({
        "track": track, "slot": 0, "plugin": "gain"
    })).expect("built-in insert must use the real engine command");
    assert_eq!(inserted["plugin"], "gain");
    assert_eq!(engine.session().track(soundcraft_model::TrackId(track)).unwrap().mixer.inserts[0].as_ref().unwrap().plugin, "gain");
    assert!(sample_peak(&engine) > 0.000001, "a built-in insert must not silently mute the track");

    assert!(engine.undo(), "plugin insert must support Undo");
    assert!(engine.session().track(soundcraft_model::TrackId(track)).unwrap().mixer.inserts[0].is_none());
    assert!(engine.redo(), "plugin insert must support Redo");
    assert_eq!(
        engine.session().find_clip(ClipId(clip)).unwrap().1.content,
        content_before_insert,
        "plugin routing must preserve MIDI note and pedal content"
    );
}

#[test]
fn orvia_4c_session_save_and_reopen_preserves_midi_pedal_and_audio() {
    let (mut engine, track, clip) = instrument_project();
    let original = engine.session().find_clip(ClipId(clip)).unwrap().1.content.clone();
    let audio_before = sample_peak(&engine);
    let folder = std::env::temp_dir().join(format!("orvia-4c-native-{}", std::process::id()));
    std::fs::create_dir_all(&folder).unwrap();
    let path = folder.join("Orvia Release Gate.scraft");
    let _ = std::fs::remove_file(&path);

    engine.execute("session.save_as", &json!({"path": path})).unwrap();
    assert!(path.exists(), "the session must be written to disk");
    let mut reopened = Engine::default();
    let opened = reopened.execute("session.open", &json!({"path": path})).unwrap();
    assert_eq!(opened["missing"], json!([]), "a self-contained MIDI test session must not have missing audio");
    let reloaded = reopened.session().track_by_name("ORVIA Keys").expect("track retained");
    assert_eq!(reloaded.id.0, track);
    assert_eq!(reopened.session().find_clip(ClipId(clip)).unwrap().1.content, original);
    let audio_after = sample_peak(&reopened);
    assert!((audio_after - audio_before).abs() < 0.00001,
        "reopened session must reproduce the synth's audio peak");

    std::fs::remove_dir_all(folder).unwrap();
}

#[test]
fn orvia_4c_recording_never_replaces_a_live_session() {
    let (mut engine, _, _) = instrument_project();
    let before = engine.session().clone();
    engine.transport.recording = true;
    for cmd in ["session.new", "session.open", "session.close", "session.revert", "app.quit"] {
        assert!(engine.execute(cmd, &json!({})).is_err(), "{cmd} cannot discard a recording");
        assert_eq!(engine.session(), &before);
    }
}
