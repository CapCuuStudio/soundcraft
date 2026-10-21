//! ORVIA's compact Cubase-inspired selected-event Info Line.
//! The main toolbar already has Select/Grab/Draw/Trim; do not duplicate it here.
//! Every edit below dispatches a real SoundCraft command; the bar never owns session data.

use crate::SoundApp;
use egui::{RichText, Ui};
use serde_json::{Value, json};
use soundcraft_model::ClipId;
use soundcraft_time::TimeFormat;

fn clip_move(id: ClipId, delta: i64) -> Value {
    json!({"clips": [id.0], "by": delta})
}

fn clip_trim_end(id: ClipId, end: i64) -> Value {
    json!({"clip": id.0, "end": end})
}

pub(crate) fn duplicate_params(id: ClipId) -> Value {
    json!({"clip": id.0})
}

pub(crate) fn split_params(id: ClipId, at: i64) -> Value {
    json!({"clip": id.0, "at": at})
}

pub fn show(app: &mut SoundApp, ui: &mut Ui) {
    let show_id = ui.id().with("orvia_info_line_open");
    let mut expanded = ui.ctx().data_mut(|d| d.get_temp::<bool>(show_id)).unwrap_or(true);
    let selected = {
        let s = app.engine.session();
        s.edit.selected_clips.first().and_then(|id| {
            s.find_clip(*id).map(|(track, clip)| {
                (
                    clip.clone(),
                    s.track(track).map_or("Track".to_string(), |t| t.name.clone()),
                    s.sample_rate.hz(),
                    s.format_position(clip.start, TimeFormat::BarsBeats),
                    soundcraft_time::format_length(
                        clip.start, clip.length, TimeFormat::BarsBeats,
                        s.sample_rate, &s.tempo, s.frame_rate,
                    ),
                )
            })
        })
    };
    ui.spacing_mut().item_spacing.x = 5.0;
    egui::ScrollArea::horizontal().id_salt("orvia_project_tools_scroll").show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new("THÔNG TIN CLIP").strong().size(11.0))
                .on_hover_text("Thông tin sự kiện Audio hoặc MIDI đang chọn");
            if ui.small_button(if expanded { "Ẩn" } else { "Hiện" }).clicked() {
                expanded = !expanded;
            }
            if !expanded {
                return;
            }
            ui.separator();
            if let Some((clip, track, rate, bars, bars_length)) = &selected {
                ui.label(RichText::new(&clip.name).strong())
                    .on_hover_text(format!("Track: {track}"));
                ui.label(format!("Nhịp {bars}")).on_hover_text(
                    format!("Bars | Beats | Ticks: {bars}. Độ dài: {bars_length}. Đồng hồ âm nhạc theo Tempo Map.")
                );
                ui.label("Bắt đầu");
                let sr = f64::from(*rate).max(1.0);
                let mut at = clip.start as f64 / sr;
                let at_field = ui.add_enabled(
                    !clip.edit_locked,
                    egui::DragValue::new(&mut at).speed(0.05).range(0.0..=14_400.0).fixed_decimals(2).suffix(" s"),
                );
                if at_field.changed() {
                    let new_start = soundcraft_time::to_samples(at * sr);
                    let delta = new_start.saturating_sub(clip.start);
                    if delta != 0 {
                        if let Err(error) = app.engine.execute_merged(
                            "edit.move_clips",
                            &clip_move(clip.id, delta),
                            &format!("orvia:clip:position:{}", clip.id.0),
                        ) {
                            app.ui.status = error.to_string();
                        }
                    }
                }
                ui.label("Độ dài");
                let mut length = clip.length as f64 / sr;
                let length_field = ui.add_enabled(
                    !clip.edit_locked,
                    egui::DragValue::new(&mut length).speed(0.05).range(0.01..=14_400.0).fixed_decimals(2).suffix(" s"),
                );
                if length_field.changed() {
                    let new_length = soundcraft_time::to_samples(length * sr).max(1);
                    let end = clip.start.saturating_add(new_length);
                    if let Err(error) = app.engine.execute_merged(
                        "edit.trim_clip",
                        &clip_trim_end(clip.id, end),
                        &format!("orvia:clip:length:{}", clip.id.0),
                    ) {
                        app.ui.status = error.to_string();
                    }
                }
                if clip.is_audio() {
                    ui.label("Gain");
                    let mut gain = clip.gain_db;
                    let response = ui.add_enabled(
                        !clip.edit_locked,
                        egui::DragValue::new(&mut gain).speed(0.2).range(-96.0..=24.0).fixed_decimals(1).suffix(" dB"),
                    );
                    if response.changed() {
                        if let Err(error) = app.engine.execute_merged(
                            "clip.gain",
                            &json!({"clips": [clip.id.0], "db": gain}),
                            &format!("orvia:clip:gain:{}", clip.id.0),
                        ) {
                            app.ui.status = error.to_string();
                        }
                    }
                }
                if ui.selectable_label(clip.muted, "Tắt tiếng").clicked() {
                    let _ = app.run("edit.mute_clips", json!({"clips": [clip.id.0]}));
                }
                if ui.add_enabled(!clip.edit_locked, egui::Button::new("Đổi tên")).clicked() {
                    app.dialogs.open_rename_clip(clip.id, &clip.name);
                }
                let locked = clip.edit_locked || clip.time_locked;
                let cursor = app.position();
                let can_split = !locked && cursor > clip.start && cursor < clip.end();
                ui.menu_button("Tác vụ", |ui| {
                    if ui.add_enabled(!locked, egui::Button::new("Nhân bản clip"))
                        .on_hover_text("Chèn một bản sao sau clip và dời các clip tiếp theo để không ghi đè")
                        .clicked()
                    {
                        let _ = app.run("clip.duplicate_one", duplicate_params(clip.id));
                        ui.close();
                    }
                    if ui.add_enabled(can_split, egui::Button::new("Chia tại con trỏ"))
                        .on_hover_text("Chỉ chia clip này tại vị trí Playhead, giữ nguyên clip khác")
                        .clicked()
                    {
                        let _ = app.run("clip.split_one", split_params(clip.id, cursor));
                        ui.close();
                    }
                    if !can_split {
                        ui.label(RichText::new("Đưa Playhead vào bên trong clip để chia").small().weak());
                    }
                    if ui.add_enabled(!locked, egui::Button::new("Căn theo lưới"))
                        .on_hover_text("Quantize clip đã chọn vào lưới nhịp hiện tại")
                        .clicked()
                    {
                        let _ = app.run("clip.quantize_to_grid", json!({"clips": [clip.id.0]}));
                        ui.close();
                    }
                    ui.separator();
                    ui.label(RichText::new("Grid/Snap và kéo mép clip dùng công cụ trên Toolbar").small().weak());
                });
            } else {
                ui.label(RichText::new("Chọn clip Audio hoặc MIDI trên Timeline để chỉnh thông tin").weak());
            }
        });
    });
    ui.ctx().data_mut(|d| d.insert_temp(show_id, expanded));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_bar_targets_only_selected_clip() {
        let id = ClipId(24);
        assert_eq!(clip_move(id, -120), json!({"clips": [24], "by": -120}));
        assert_eq!(clip_trim_end(id, 48000), json!({"clip": 24, "end": 48000}));
        assert_eq!(duplicate_params(id), json!({"clip": 24}));
        assert_eq!(split_params(id, 24000), json!({"clip": 24, "at": 24000}));
    }
}
