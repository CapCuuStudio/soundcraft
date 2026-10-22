//! ORVIA 3B: genuine MIDI controller lane sharing the Key Editor timeline.
use crate::SoundApp;
use crate::theme::Tokens;
use egui::{Align, Color32, Layout, Rect, Sense, Stroke, Ui, UiBuilder, pos2, vec2};
use serde_json::json;
use soundcraft_midi::{CtrlKind, Sequence};
use soundcraft_model::ClipId;

const VELOCITY: u8 = 255;
const LANES: [(u8, &str); 6] = [
    (VELOCITY, "Velocity"),
    (1, "CC1 Modulation"),
    (7, "CC7 Volume"),
    (10, "CC10 Pan"),
    (11, "CC11 Expression"),
    (64, "CC64 Sustain"),
];

fn lane_label(number: u8) -> &'static str {
    LANES.iter().find(|(n, _)| *n == number).map_or("Velocity", |(_, label)| *label)
}

pub(super) fn show(
    app: &mut SoundApp, ui: &mut Ui, clip: ClipId, sequence: &Sequence,
    roll: Rect, header: Rect, area: Rect, color: Color32,
    grid_ticks: i64, len_ticks: i64, locked: bool,
) {
    let id = ui.id().with(("orvia3b-controller", clip.0));
    let mut lane = ui.ctx().data(|d| d.get_temp::<u8>(id)).unwrap_or(VELOCITY);
    let mut top = ui.new_child(UiBuilder::new().max_rect(header).layout(Layout::left_to_right(Align::Center)));
    top.spacing_mut().item_spacing.x = 8.0;
    top.strong("MIDI CC");
    egui::ComboBox::from_id_salt(("orvia3b-picker", clip.0))
        .selected_text(lane_label(lane)).width(130.0)
        .show_ui(&mut top, |ui| {
            for (number, label) in LANES {
                ui.selectable_value(&mut lane, number, label);
            }
        });
    top.label(egui::RichText::new(if locked { "Clip đang khóa" }
        else if lane == VELOCITY { "Kéo để chỉnh lực nhấn nốt" }
        else { "Click/kéo: vẽ điểm · Chuột phải: xóa điểm · MIDI channel 1" })
        .small().color(Tokens::current().text_dim));
    ui.ctx().data_mut(|d| d.insert_temp(id, lane));

    ui.painter().rect_filled(area, 0.0, Color32::from_rgb(28, 28, 30));
    let response = ui.interact(area, ui.id().with(("orvia3b-lane", clip.0, lane)), Sense::click_and_drag());
    let painter = ui.painter().with_clip_rect(area);
    let px_per_tick = roll.width() / (len_ticks.max(1) as f32);
    let x_of = |tick: i64| roll.min.x + tick as f32 * px_per_tick;
    let value_of = |y: f32| (((area.max.y - 2.0 - y) / (area.height() - 4.0)) * 127.0)
        .round().clamp(0.0, 127.0) as i64;

    if lane == VELOCITY {
        for (index, note) in sequence.notes.iter().enumerate() {
            let x = x_of(note.start);
            let y = area.max.y - 2.0 - (area.height() - 4.0) * f32::from(note.velocity) / 127.0;
            let ink = if app.midi.selected.contains(&index) { Color32::WHITE } else { color };
            painter.line_segment([pos2(x, area.max.y - 2.0), pos2(x, y)], Stroke::new(3.0, ink));
            painter.circle_filled(pos2(x, y), 2.5, ink);
        }
        if !locked && (response.clicked() || response.dragged())
            && let Some(p) = response.interact_pointer_pos()
            && let Some((index, note)) = sequence.notes.iter().enumerate()
                .min_by_key(|(_, n)| ((x_of(n.start) - p.x).abs() * 10.0) as i64)
            && (x_of(note.start) - p.x).abs() < 8.0
        {
            if let Err(err) = app.engine.execute_merged("midi.note_edit",
                &json!({"clip":clip.0,"index":index,"velocity":value_of(p.y).max(1)}),
                &format!("orvia3b-velocity:{}", clip.0)) {
                app.ui.status = err.to_string();
            }
        }
    } else {
        for event in sequence.ctrls.iter()
            .filter(|event| event.kind == CtrlKind::Cc(lane) && event.channel == 0)
        {
            let x = x_of(event.tick);
            let y = area.max.y - 2.0 - (area.height() - 4.0) * event.value.clamp(0, 127) as f32 / 127.0;
            painter.line_segment([pos2(x, area.max.y - 2.0), pos2(x, y)], Stroke::new(2.5, color));
            painter.circle_filled(pos2(x, y), 3.0, Color32::WHITE);
        }
        if !locked && (response.clicked() || response.dragged())
            && let Some(p) = response.interact_pointer_pos()
        {
            let tick = ((p.x - roll.min.x) / px_per_tick / grid_ticks.max(1) as f32)
                .round() as i64 * grid_ticks.max(1);
            let tick = tick.clamp(0, len_ticks.saturating_sub(1));
            if let Err(err) = app.engine.execute_merged("midi.cc_set",
                &json!({"clip":clip.0,"cc":lane,"tick":tick,"value":value_of(p.y),"channel":0}),
                &format!("orvia3b-cc:{}:{lane}", clip.0)) {
                app.ui.status = err.to_string();
            }
        }
        if !locked && response.clicked_by(egui::PointerButton::Secondary)
            && let Some(p) = response.interact_pointer_pos()
            && let Some(event) = sequence.ctrls.iter()
                .filter(|event| event.kind == CtrlKind::Cc(lane) && event.channel == 0)
                .min_by_key(|event| ((x_of(event.tick) - p.x).abs() * 10.0) as i64)
            && (x_of(event.tick) - p.x).abs() <= 7.0
        {
            let _ = app.run("midi.cc_delete",
                json!({"clip":clip.0,"cc":lane,"tick":event.tick,"channel":0}));
        }
    }
    if response.drag_stopped() {
        app.engine.end_merge();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lane_names_include_pedal_and_velocity() {
        assert_eq!(lane_label(64), "CC64 Sustain");
        assert_eq!(lane_label(255), "Velocity");
        assert_eq!(LANES.len(), 6);
    }
}
