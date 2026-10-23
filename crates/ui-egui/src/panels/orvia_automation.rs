//! ORVIA 3D — practical, non-destructive track automation editing.
//! Points are stored in Track::automation via existing undoable engine commands.
use crate::SoundApp;
use crate::theme::Tokens;
use egui::{RichText, Ui};
use serde_json::json;
use soundcraft_model::{AutoParam, Track, TrackKind};

fn options(track: &Track) -> Vec<(&'static str, AutoParam)> {
    let mut result = vec![("Volume", AutoParam::Volume)];
    if !track.mixer.pan.is_empty() {
        result.push(("Pan", AutoParam::Pan(0)));
        if track.mixer.pan.len() > 1 {
            result.push(("Pan R", AutoParam::Pan(1)));
        }
    }
    result.push(("Mute", AutoParam::Mute));
    result
}

fn param_id(param: &AutoParam) -> &'static str {
    match param {
        AutoParam::Volume => "volume",
        AutoParam::Pan(0) => "pan",
        AutoParam::Pan(_) => "pan2",
        AutoParam::Mute => "mute",
        _ => "volume",
    }
}

fn value_range(param: &AutoParam) -> (f32, f32) {
    match param {
        AutoParam::Volume => (-60.0, 12.0),
        AutoParam::Pan(_) => (-1.0, 1.0),
        AutoParam::Mute => (0.0, 1.0),
        _ => (0.0, 1.0),
    }
}

fn default_value(track: &Track, param: &AutoParam) -> f32 {
    match param {
        AutoParam::Volume => track.mixer.volume_db.clamp(-60.0, 12.0),
        AutoParam::Pan(index) => track.mixer.pan.get(usize::from(*index)).copied().unwrap_or(0.0),
        AutoParam::Mute => if track.mixer.mute { 1.0 } else { 0.0 },
        _ => 0.0,
    }
}

fn can_automate(track: &Track) -> bool {
    track.kind.is_audio_path() && !track.inactive && !track.frozen
}

pub(super) fn show(app: &mut SoundApp, ui: &mut Ui, track: &Track) {
    let dim = Tokens::current().text_dim;
    if !can_automate(track) {
        ui.label(RichText::new("Track này không có luồng âm thanh đang chỉnh được.")
            .small().color(dim));
        return;
    }
    let opts = options(track);
    let pick_key = egui::Id::new(("orvia-3d-param", track.id.0));
    let mut chosen = ui.ctx().data(|d| d.get_temp::<String>(pick_key))
        .filter(|p| opts.iter().any(|(_, value)| param_id(value) == p))
        .unwrap_or_else(|| {
            let from_view = AutoParam::parse(&track.view);
            from_view.filter(|p| opts.iter().any(|(_, v)| v == p))
                .map_or_else(|| "volume".to_string(), |p| param_id(&p).to_string())
        });
    ui.horizontal(|ui| {
        ui.label("Thông số");
        let label = opts.iter().find(|(_, p)| param_id(p) == chosen)
            .map_or("Volume", |(name, _)| *name);
        egui::ComboBox::from_id_salt(("orvia-3d-options", track.id.0))
            .selected_text(label)
            .show_ui(ui, |ui| {
                for (name, p) in &opts {
                    ui.selectable_value(&mut chosen, param_id(p).to_string(), *name);
                }
            });
    });
    ui.ctx().data_mut(|d| d.insert_temp(pick_key, chosen.clone()));
    let param = opts.iter().find(|(_, p)| param_id(p) == chosen)
        .map_or(AutoParam::Volume, |(_, p)| p.clone());
    let param_text = param_id(&param);
    let (lo, hi) = value_range(&param);
    let points = track.lane(&param).map(|lane| lane.points.clone()).unwrap_or_default();
    let insertion = app.engine.session().edit.selection.start.max(0);
    let selection = app.engine.session().edit.selection;
    let samples_per_sec = app.engine.session().sample_rate.as_f64().max(1.0);

    ui.horizontal(|ui| {
        let showing = AutoParam::parse(&track.view).as_ref() == Some(&param);
        if ui.small_button(if showing { "Đang xem Lane" } else { "Hiện Lane trên Timeline" }).clicked() && !showing {
            let _ = app.run("track.view", json!({"track":track.id.0, "view":param_text}));
        }
        if showing && ui.small_button("Về Clips").clicked() {
            let view = if track.kind == TrackKind::Instrument { "notes" } else { "waveform" };
            let _ = app.run("track.view", json!({"track":track.id.0, "view":view}));
        }
    });
    ui.label(RichText::new("Dùng Pencil trên Lane để vẽ điểm; bảng này cho phép chỉnh chính xác và xóa từng điểm.")
        .small().color(dim));
    ui.separator();

    // A new point is located at the edit cursor; it never moves or deletes any clip.
    let value_key = egui::Id::new(("orvia-3d-value", track.id.0, param_text));
    let mut value = ui.ctx().data(|d| d.get_temp::<f32>(value_key))
        .unwrap_or_else(|| default_value(track, &param));
    ui.horizontal(|ui| {
        ui.label(format!("@ {:.2}s", insertion as f64 / samples_per_sec));
        if param == AutoParam::Mute {
            let mut enabled = value >= 0.5;
            if ui.checkbox(&mut enabled, "Mute").changed() { value = if enabled { 1.0 } else { 0.0 }; }
        } else {
            ui.add(egui::Slider::new(&mut value, lo..=hi).show_value(true)
                .suffix(if param == AutoParam::Volume { " dB" } else { "" }));
        }
    });
    ui.ctx().data_mut(|d| d.insert_temp(value_key, value));
    ui.horizontal(|ui| {
        if ui.small_button("+ Thêm điểm tại con trỏ").clicked() {
            if let Err(err) = app.run("automation.set_point",
                json!({"track":track.id.0,"param":param_text,"at":insertion,"value":value})) {
                app.ui.status = err;
            }
        }
        if !selection.is_empty() && ui.small_button("Ghi vào vùng chọn").clicked() {
            if let Err(err) = app.run("automation.write_range",
                json!({"track":track.id.0,"param":param_text,"start":selection.start,
                        "end":selection.end,"value":value})) {
                app.ui.status = err;
            }
        }
    });
    ui.label(RichText::new(format!("{} điểm · chỉnh từng điểm bên dưới", points.len()))
        .small().color(dim));
    egui::ScrollArea::vertical().id_salt(("orvia-3d-point-list", track.id.0))
        .max_height(175.0).show(ui, |ui| {
            if points.is_empty() {
                ui.label(RichText::new("Chưa có điểm. Chọn vị trí trên Timeline rồi nhấn Thêm.")
                    .small().color(dim));
            }
            for pt in points {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(format!("{:.2}s", pt.at as f64 / samples_per_sec))
                        .monospace().small());
                    if param == AutoParam::Mute {
                        let mut muted = pt.value >= 0.5;
                        if ui.checkbox(&mut muted, "Mute").changed() {
                            let _ = app.run("automation.set_point", json!({
                                "track":track.id.0,"param":param_text,"at":pt.at,
                                "value":if muted { 1.0 } else { 0.0 }
                            }));
                        }
                    } else {
                        let mut v = pt.value.clamp(lo, hi);
                        let response = ui.add(egui::DragValue::new(&mut v)
                            .range(lo..=hi).speed(if param == AutoParam::Volume { 0.1 } else { 0.01 })
                            .max_decimals(2));
                        if response.changed() {
                            if let Err(err) = app.engine.execute_merged("automation.set_point",
                                &json!({"track":track.id.0,"param":param_text,"at":pt.at,"value":v}),
                                &format!("orvia-3d-point:{}:{}:{}", track.id.0, param_text, pt.at)) {
                                app.ui.status = err.to_string();
                            }
                        }
                        if response.drag_stopped() || response.lost_focus() { app.engine.end_merge(); }
                    }
                    if ui.small_button("×").on_hover_text("Chỉ xóa điểm này, giữ các điểm khác").clicked() {
                        if let Err(err) = app.run("automation.clear", json!({
                            "track":track.id.0,"param":param_text,
                            "start":pt.at,"end":pt.at.saturating_add(1)
                        })) {
                            app.ui.status = err;
                        }
                    }
                });
            }
        });
    ui.label(RichText::new("Điểm lưu vào dự án, hỗ trợ Undo/Redo. MIDI CC nằm riêng trong Piano Roll.")
        .small().color(dim));
}

#[cfg(test)]
mod tests {
    use super::*;
    use soundcraft_model::{ChannelFormat, Session};
    #[test]
    fn stage3d_volume_pan_mute_choices_respect_channel_and_track_type() {
        let mut session = Session::default();
        let audio = session.add_track(TrackKind::Audio, ChannelFormat::Mono, Some("Audio"));
        let inst = session.add_track(TrackKind::Instrument, ChannelFormat::Stereo, Some("Synth"));
        let midi = session.add_track(TrackKind::Midi, ChannelFormat::Mono, Some("MIDI"));
        let a = session.track(audio).unwrap();
        let s = session.track(inst).unwrap();
        assert!(can_automate(a));
        assert!(can_automate(s));
        assert!(!can_automate(session.track(midi).unwrap()));
        assert!(options(a).iter().any(|(_, p)| *p == AutoParam::Volume));
        assert_eq!(param_id(&AutoParam::Pan(1)), "pan2");
        assert_eq!(value_range(&AutoParam::Volume), (-60.0, 12.0));
    }
}
