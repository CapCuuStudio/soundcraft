//! ORVIA Studio Rack v1 — local visual tokens; no audio logic.
use egui::{Color32, Frame, Margin, Stroke, Ui};
use crate::theme::{self, Tokens};

#[derive(Clone, Copy)]
pub(super) struct Skin {
    pub surface: Color32,
    pub raised: Color32,
    pub border: Color32,
    pub text: Color32,
    pub secondary: Color32,
    pub accent: Color32,
    pub ink: Color32,
}

impl Skin {
    pub(super) fn current() -> Self {
        if theme::is_light() {
            let t = Tokens::current();
            Self { surface:t.panel_bg,raised:t.panel_bg2,border:t.border_light,
                text:t.text,secondary:t.text_dim,accent:t.accent,ink:Color32::WHITE }
        } else {
            Self {
                surface:Color32::from_rgb(23,33,43),
                raised:Color32::from_rgb(32,45,57),
                border:Color32::from_rgb(53,70,83),
                text:Color32::from_rgb(229,237,244),
                secondary:Color32::from_rgb(170,185,198),
                accent:Color32::from_rgb(71,215,209),
                ink:Color32::from_rgb(11,37,48),
            }
        }
    }
    pub(super) fn card(self, ui:&mut Ui, add: impl FnOnce(&mut Ui)) {
        Frame::new().fill(self.raised).stroke(Stroke::new(1.0,self.border))
            .corner_radius(8).inner_margin(Margin::same(8))
            .show(ui,|ui| { ui.set_min_width(ui.available_width()); add(ui); });
    }
}
