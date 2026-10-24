//! ORVIA Stage 3C: a usable, truthful rack for SoundCraft's built-in processors.
//! Native third-party hosts remain available only through their existing desktop menus.
use super::rack_skin::Skin;
use crate::SoundApp;
use crate::theme::Tokens;
use egui::{Ui, RichText};
use serde_json::json;
use soundcraft_dsp::{Category, PluginInfo};
use soundcraft_model::{Track, TrackId, TrackKind};

fn accepts(track: &Track, instruments: bool) -> bool {
    if instruments { track.kind == TrackKind::Instrument } else { track.kind.is_audio_path() }
}

fn free_slots(track: &Track) -> Vec<usize> {
    track.mixer.inserts.iter().enumerate().filter_map(|(i, ins)| ins.is_none().then_some(i)).collect()
}

fn catalog(instruments: bool, search: &str, category: Option<Category>) -> Vec<&'static PluginInfo> {
    let needle = search.trim().to_lowercase();
    soundcraft_dsp::plugins().iter().copied()
        .filter(|info| info.is_instrument == instruments)
        .filter(|info| category.is_none_or(|cat| info.category == cat))
        .filter(|info| needle.is_empty() || info.name.to_lowercase().contains(&needle) || info.id.contains(&needle))
        .collect()
}

fn category_name(cat: Category) -> &'static str {
    match cat {
        Category::Eq => "EQ",
        Category::Dynamics => "Dynamics",
        Category::PitchShift => "Pitch",
        Category::Reverb => "Reverb",
        Category::Delay => "Delay",
        Category::Modulation => "Modulation",
        Category::Harmonic => "Harmonic",
        Category::Dither => "Dither",
        Category::Instrument => "Nhạc cụ",
        Category::Other => "Khác",
    }
}

fn open_editor(app: &mut SoundApp, track: TrackId, slot: usize) {
    if !app.ui.plugin_windows.contains(&(track, slot)) {
        app.ui.plugin_windows.push((track, slot));
    }
}

pub(super) fn show(app: &mut SoundApp, ui: &mut Ui) {
    let t = Tokens::current();
    let skin = Skin::current();
    ui.spacing_mut().item_spacing.y = 7.0;
    ui.painter().rect_filled(ui.max_rect(), 0.0, skin.surface);
    ui.spacing_mut().button_padding = egui::vec2(8.0, 5.0);
    ui.visuals_mut().widgets.inactive.bg_fill = skin.raised;
    ui.visuals_mut().widgets.inactive.fg_stroke.color = skin.text;
    ui.visuals_mut().selection.bg_fill = skin.accent;
    ui.horizontal(|ui| {
        ui.label(RichText::new("STUDIO RACK").size(13.0).strong().color(skin.text));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(RichText::new("ORVIA").size(11.0).strong().color(skin.accent));
        });
    });
    let mode_id = egui::Id::new("orvia_3c_rack_mode");
    let filter_id = egui::Id::new("orvia_3c_rack_filter");
    let cat_id = egui::Id::new("orvia_3c_rack_category");
    let info_id = egui::Id::new("orvia_3c_rack_details");
    let instruments = ui.ctx().data(|d| d.get_temp::<bool>(mode_id)).unwrap_or_else(|| {
        let s = app.engine.session();
        s.edit.selected_tracks.first().and_then(|id| s.track(*id))
            .is_some_and(|t| t.kind == TrackKind::Instrument)
    });
    let mut instruments = instruments;
    ui.columns(2, |cols| {
        for (i, (mode, title)) in [(true, "Nhạc cụ"), (false, "Hiệu ứng")].iter().enumerate() {
            let active = instruments == *mode;
            if cols[i].add_sized([cols[i].available_width(), 30.0],
                egui::Button::new(*title)
                    .fill(if active { skin.accent } else { skin.raised })
                    .corner_radius(5)).clicked() {
                instruments = *mode;
            }
        }
    });
    ui.ctx().data_mut(|d| d.insert_temp(mode_id, instruments));
    let mut search = ui.ctx().data(|d| d.get_temp::<String>(filter_id)).unwrap_or_default();
    ui.add(egui::TextEdit::singleline(&mut search).hint_text("Tìm plugin tích hợp...").desired_width(f32::INFINITY));
    ui.ctx().data_mut(|d| d.insert_temp(filter_id, search.clone()));

    let (selected, tracks): (Option<Track>, Vec<(TrackId, String, TrackKind)>) = {
        let s = app.engine.session();
        let chosen = s.edit.selected_tracks.first().and_then(|id| s.track(*id)).cloned();
        let tracks = s.tracks.iter().filter(|tr| !tr.hidden)
            .map(|tr| (tr.id, tr.name.clone(), tr.kind)).collect();
        (chosen, tracks)
    };
    ui.horizontal(|ui| {
        ui.label("Track:");
        let title = selected.as_ref().map_or("Chọn Track", |track| track.name.as_str());
        egui::ComboBox::from_id_salt("orvia_3c_track_select")
            .selected_text(title)
            .width(ui.available_width().max(70.0))
            .show_ui(ui, |ui| {
                for (id, name, kind) in &tracks {
                    if ui.selectable_label(selected.as_ref().is_some_and(|tr| tr.id == *id),
                        format!("{name} · {}", kind.label())).clicked() {
                        let _ = app.run("edit.select", json!({"tracks": [id.0], "exact": true}));
                    }
                }
            });
    });
    let compatible = selected.as_ref().is_some_and(|track| accepts(track, instruments));
    if !compatible {
        let reason = if instruments { "Chọn Instrument Track để gán nhạc cụ." }
                     else { "Chọn Audio, Instrument, Aux hoặc Master Track để thêm hiệu ứng." };
        ui.label(RichText::new(reason).color(t.text_dim).small());
        if instruments && ui.small_button("+ Tạo Instrument Track").clicked() {
            let _ = app.run("track.new", json!({"kind":"instrument"}));
        }
    }

    ui.separator();
    if cfg!(target_arch = "wasm32") {
        ui.label(RichText::new("WEB · Chỉ plugin tích hợp, không chạy Kontakt/VST3/CLAP native.")
            .small().color(t.text_dim));
    } else {
        ui.label(RichText::new("Tích hợp sẵn · Plugin VST3/CLAP/AU có thể chọn qua MixConsole desktop.")
            .small().color(t.text_dim));
    }

    let mut slot = None;
    if !instruments {
        if let Some(track) = selected.as_ref().filter(|track| accepts(track, false)) {
            egui::CollapsingHeader::new("INSERT RACK · Hiệu ứng đã gán")
                .default_open(true)
                .show(ui, |ui| {
                    for (index, insert) in track.mixer.inserts.iter().enumerate() {
                        let Some(insert) = insert.as_ref() else { continue };
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(format!("{:02}", index + 1)).small().monospace());
                            let name = soundcraft_dsp::plugin_info(&insert.plugin)
                                .map_or(insert.plugin.as_str(), |p| p.name);
                            let resp = ui.add_sized([ui.available_width().max(75.0) - 88.0, 19.0],
                                egui::Button::new(name));
                            if resp.on_hover_text("Mở tham số hiệu ứng").clicked() {
                                open_editor(app, track.id, index);
                            }
                            if ui.small_button(if insert.bypass { "OFF" } else { "ON" })
                                .on_hover_text("Bypass/bật hiệu ứng").clicked() {
                                let _ = app.run("mix.insert_bypass", json!({"track": track.id.0, "slot":index}));
                            }
                            if ui.small_button("×").on_hover_text("Xóa hiệu ứng khỏi slot").clicked() {
                                if app.run("mix.insert_remove", json!({"track": track.id.0, "slot":index})).is_ok() {
                                    app.ui.plugin_windows.retain(|(id, s)| *id != track.id || *s != index);
                                }
                            }
                        });
                    }
                });
            let free = free_slots(track);
            let slot_key = egui::Id::new(("orvia_3c_insert_slot", track.id.0));
            let mut selected_slot = ui.ctx().data(|d| d.get_temp::<usize>(slot_key))
                .filter(|s| free.contains(s))
                .or_else(|| free.first().copied());
            if let Some(ref mut index) = selected_slot {
                ui.horizontal(|ui| {
                    ui.label("Thêm vào:");
                    egui::ComboBox::from_id_salt(("orvia_3c_slot_choice", track.id.0))
                        .selected_text(format!("Insert {:02}", *index + 1))
                        .show_ui(ui, |ui| {
                            for free_index in &free {
                                ui.selectable_value(index, *free_index, format!("Insert {:02}", free_index + 1));
                            }
                        });
                });
                ui.ctx().data_mut(|d| d.insert_temp(slot_key, *index));
            } else {
                ui.label(RichText::new("Đã dùng hết 10 vị trí Insert. Xóa hiệu ứng để thêm mới.")
                    .small().color(t.text_dim));
            }
            slot = selected_slot;
        }
    } else if let Some(track) = selected.as_ref().filter(|track| accepts(track, true)) {
        let current = track.instrument.as_ref().map(|ins| ins.plugin.as_str()).unwrap_or("Không có");
        ui.label(RichText::new(format!("Nhạc cụ hiện tại: {current}")).small().color(t.text_dim));
        if track.instrument.is_some() && ui.small_button("Mở bảng điều chỉnh nhạc cụ").clicked() {
            open_editor(app, track.id, soundcraft_mix::INSTRUMENT_SLOT);
        }
        if current == "sampler" {
            ui.label(RichText::new("Sampler cần tải file âm thanh trước khi phát.")
                .small().color(t.text_dim));
        }
    }

    let mut category = ui.ctx().data(|d| d.get_temp::<Option<Category>>(cat_id)).flatten();
    if !instruments {
        let mut cats = Vec::new();
        for p in soundcraft_dsp::plugins().iter().filter(|p| !p.is_instrument) {
            if !cats.contains(&p.category) { cats.push(p.category); }
        }
        ui.horizontal(|ui| {
            ui.label("Nhóm:");
            egui::ComboBox::from_id_salt("orvia_3c_category")
                .selected_text(category.map_or("Tất cả", category_name))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut category, None, "Tất cả");
                    for c in cats {
                        ui.selectable_value(&mut category, Some(c), category_name(c));
                    }
                });
        });
        ui.ctx().data_mut(|d| d.insert_temp(cat_id, category));
    }
    let plugins = catalog(instruments, &search, if instruments { None } else { category });
    ui.label(RichText::new(format!("{} plugin tích hợp", plugins.len())).small().color(t.text_dim));

    let details = ui.ctx().data(|d| d.get_temp::<String>(info_id)).unwrap_or_default();
    if let Some(info) = soundcraft_dsp::plugin_info(&details).filter(|p| p.is_instrument == instruments) {
        egui::CollapsingHeader::new(format!("Chi tiết · {}", info.name)).default_open(true)
            .show(ui, |ui| {
                ui.label(RichText::new(format!("ID: {} · {}", info.id, category_name(info.category))).small());
                if info.id == "sampler" {
                    ui.label(RichText::new("Cần nạp sample WAV/AIFF... trong cửa sổ Sampler.").small());
                }
                for p in info.params.iter().take(6) {
                    ui.label(RichText::new(format!("{}: {}", p.name, p.format(p.default))).small().color(t.text_dim));
                }
                if info.params.len() > 6 {
                    ui.label(RichText::new("Các tham số còn lại nằm trong cửa sổ plugin.").small().color(t.text_dim));
                }
            });
    }

    egui::ScrollArea::vertical().id_salt("orvia_3c_rack_catalog").show(ui, |ui| {
        if plugins.is_empty() {
            ui.label("Không tìm thấy plugin tích hợp phù hợp.");
        }
        for info in plugins {
            let same = instruments && selected.as_ref().and_then(|tr| tr.instrument.as_ref())
                .is_some_and(|ins| ins.plugin == info.id);
            let can_assign = compatible && (instruments || slot.is_some());
            ui.horizontal(|ui| {
                let name = ui.add_sized([ui.available_width().max(125.0) - 66.0, 24.0],
                    egui::Button::new(info.name).selected(details == info.id));
                if name.on_hover_text(format!("Xem thông số · {}", info.id)).clicked() {
                    ui.ctx().data_mut(|d| d.insert_temp(info_id, info.id.to_string()));
                }
                let action = if same { "Chỉnh" } else if instruments { "Gán" } else { "Thêm" };
                let assign = ui.add_enabled(can_assign, egui::Button::new(action).min_size(egui::vec2(55.0, 24.0)));
                if assign.clicked() {
                    if let Some(track) = selected.as_ref() {
                        if same {
                            open_editor(app, track.id, soundcraft_mix::INSTRUMENT_SLOT);
                        } else {
                            let (command, args, editor_slot) = if instruments {
                                ("mix.instrument", json!({"track":track.id.0,"plugin":info.id}),
                                    soundcraft_mix::INSTRUMENT_SLOT)
                            } else {
                                let index = slot.unwrap_or(0);
                                ("mix.insert", json!({"track":track.id.0,"slot":index,"plugin":info.id}), index)
                            };
                            if app.run(command, args).is_ok() {
                                open_editor(app, track.id, editor_slot);
                            }
                        }
                    }
                }
            });
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use soundcraft_model::{ChannelFormat, Insert, Session};

    #[test]
    fn stage3c_catalog_is_real_builtin_and_classified() {
        let instruments = catalog(true, "", None);
        assert!(instruments.iter().any(|p| p.id == "subtractive_synth"));
        assert!(instruments.iter().any(|p| p.id == "drum_synth"));
        assert!(instruments.iter().all(|p| p.is_instrument && soundcraft_dsp::plugin_info(p.id).is_some()));
        let fx = catalog(false, "reverb", Some(Category::Reverb));
        assert!(fx.iter().any(|p| p.id == "room_reverb"));
        assert!(fx.iter().all(|p| !p.is_instrument && p.category == Category::Reverb));
        assert!(catalog(false, "vst3:kontakt", None).is_empty());
    }

    #[test]
    fn stage3c_track_type_and_insert_free_slot_safety() {
        let mut session = Session::default();
        let instrument = session.add_track(TrackKind::Instrument, ChannelFormat::Stereo, Some("Keys"));
        let audio = session.add_track(TrackKind::Audio, ChannelFormat::Stereo, Some("Vox"));
        assert!(accepts(session.track(instrument).unwrap(), true));
        assert!(!accepts(session.track(audio).unwrap(), true));
        assert!(accepts(session.track(audio).unwrap(), false));
        let t = session.track_mut(audio).unwrap();
        t.mixer.inserts[0] = Some(Insert::new("compressor"));
        assert_eq!(free_slots(t).first().copied(), Some(1));
        assert!(!free_slots(t).contains(&0), "an existing effect must not be overwritten");
    }
}
