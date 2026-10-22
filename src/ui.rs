//! Small UI building blocks in the main app's style (buttons, cards,
//! notices, badges).

use crate::icons::{self, Icon};
use crate::theme;
use eframe::egui::{
    self, Color32, Frame, Margin, Response, RichText, Rounding, Sense, Stroke, Ui, Vec2, WidgetText,
};

// --- Schedulo master accent: Electric Blue (brand book: #1E4BFF) ----------
// White on Electric Blue is 6.0:1, so text on the accent is white.
pub const ACCENT: Color32 = theme::BRAND_ELECTRIC;
pub const ACCENT_HOVER: Color32 = theme::ACCENT_HOVER;
pub const ACCENT_PRESSED: Color32 = theme::ACCENT_DARK;
pub const ACCENT_DARK: Color32 = theme::ACCENT_DARK;
pub const ACCENT_SOFT: Color32 = theme::ACCENT_SOFT;
pub const ACCENT_DISABLED: Color32 = theme::ACCENT_DISABLED;
pub const ON_ACCENT: Color32 = Color32::WHITE;

/// The shared light style with the Schedulo accent (Electric Blue).
pub fn apply_style(ctx: &egui::Context) {
    theme::apply_style(ctx);
    ctx.style_mut(|s| {
        let v = &mut s.visuals;
        v.hyperlink_color = ACCENT_DARK;
        v.selection.bg_fill = ACCENT_SOFT;
        v.selection.stroke = theme::stroke(1.0, ACCENT_DARK);
        v.widgets.active.bg_stroke = theme::stroke(1.0, ACCENT);
    });
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Normal,
    Primary,
    Danger,
    Ghost,
}

/// Button with an optional vector icon.
pub fn button_ex(
    ui: &mut Ui,
    icon: Option<Icon>,
    label: &str,
    kind: Kind,
    enabled: bool,
) -> Response {
    let font = if kind == Kind::Primary {
        theme::semibold(13.5)
    } else {
        theme::regular(13.5)
    };
    let galley = ui
        .painter()
        .layout_no_wrap(label.to_string(), font.clone(), theme::TEXT);
    let icon_w = if icon.is_some() { 22.0 } else { 0.0 };
    let height = 28.0;
    let width = (22.0 + icon_w + galley.size().x).max(height);
    let sense = if enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(width, height), sense);
    if ui.is_rect_visible(rect) {
        let hovered = enabled && resp.hovered();
        let pressed = enabled && resp.is_pointer_button_down_on();
        let (fill, stroke, fg) = match kind {
            Kind::Primary => (
                if pressed {
                    ACCENT_PRESSED
                } else if hovered {
                    ACCENT_HOVER
                } else {
                    ACCENT
                },
                Stroke::NONE,
                ON_ACCENT,
            ),
            Kind::Danger => (
                if hovered {
                    theme::ERROR_SOFT
                } else {
                    theme::CARD
                },
                theme::stroke(
                    1.0,
                    if hovered {
                        theme::ERROR
                    } else {
                        theme::BORDER_STRONG
                    },
                ),
                theme::ERROR,
            ),
            Kind::Normal => (
                if pressed {
                    Color32::from_rgb(0xE1, 0xE7, 0xF6)
                } else if hovered {
                    Color32::from_rgb(0xEF, 0xF2, 0xFA)
                } else {
                    theme::CARD
                },
                theme::stroke(
                    1.0,
                    if hovered {
                        Color32::from_rgb(0xA4, 0xB0, 0xCC)
                    } else {
                        theme::BORDER_STRONG
                    },
                ),
                theme::TEXT,
            ),
            Kind::Ghost => (
                if hovered {
                    Color32::from_rgb(0xE8, 0xEC, 0xF7)
                } else {
                    Color32::TRANSPARENT
                },
                Stroke::NONE,
                theme::TEXT,
            ),
        };
        let (fill, fg) = if enabled {
            (fill, fg)
        } else if kind == Kind::Primary {
            (ACCENT_DISABLED, theme::TEXT_MUTED)
        } else {
            (fill, theme::TEXT_FAINT)
        };
        ui.painter().rect(rect, 5.0, fill, stroke);
        let mut x = rect.left() + 11.0;
        if let Some(ic) = icon {
            let ir = egui::Rect::from_center_size(
                egui::pos2(x + 8.0, rect.center().y),
                Vec2::splat(16.0),
            );
            icons::paint(ui.painter(), ir, ic, fg);
            x += icon_w;
        }
        let galley = ui.painter().layout_no_wrap(label.to_string(), font, fg);
        let pos = egui::pos2(x, rect.center().y - galley.size().y / 2.0);
        ui.painter().galley(pos, galley, fg);
    }
    if enabled {
        resp.on_hover_cursor(egui::CursorIcon::PointingHand)
    } else {
        resp
    }
}

pub fn button(ui: &mut Ui, icon: Icon, label: &str) -> Response {
    button_ex(ui, Some(icon), label, Kind::Normal, true)
}

/// White card with a subtle border.
pub fn card<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    Frame::none()
        .fill(theme::CARD)
        .stroke(theme::stroke(1.0, theme::BORDER))
        .rounding(Rounding::same(theme::ROUNDING))
        .inner_margin(Margin::same(16.0))
        .show(ui, add)
        .inner
}

/// Card heading.
pub fn card_title(ui: &mut Ui, icon: Icon, title: &str) {
    ui.horizontal(|ui| {
        icons::icon(ui, icon, 18.0, ACCENT_DARK);
        ui.label(
            RichText::new(title)
                .font(theme::semibold(15.0))
                .color(theme::TEXT),
        );
    });
    ui.add_space(6.0);
}

/// A notice strip with an icon.
pub fn notice(ui: &mut Ui, icon: Icon, fg: Color32, bg: Color32, text: impl Into<WidgetText>) {
    Frame::none()
        .fill(bg)
        .rounding(Rounding::same(5.0))
        .inner_margin(Margin::symmetric(10.0, 7.0))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                icons::icon(ui, icon, 16.0, fg);
                ui.label(text);
            });
        });
}

pub fn error(ui: &mut Ui, text: impl Into<WidgetText>) {
    notice(ui, Icon::Error, theme::ERROR, theme::ERROR_SOFT, text);
}

pub fn info(ui: &mut Ui, text: impl Into<WidgetText>) {
    notice(ui, Icon::Info, theme::INFO, theme::INFO_SOFT, text);
}

pub fn warning(ui: &mut Ui, text: impl Into<WidgetText>) {
    notice(ui, Icon::Warning, theme::WARNING, theme::WARNING_SOFT, text);
}

/// Small rounded label with coloured background.
pub fn badge(ui: &mut Ui, text: &str, fg: Color32, bg: Color32) -> Response {
    let galley = ui
        .painter()
        .layout_no_wrap(text.to_string(), theme::semibold(11.5), fg);
    let size = Vec2::new(galley.size().x + 14.0, 20.0);
    let (rect, resp) = ui.allocate_exact_size(size, Sense::hover());
    ui.painter().rect_filled(rect, 10.0, bg);
    ui.painter().galley(
        egui::pos2(rect.left() + 7.0, rect.center().y - galley.size().y / 2.0),
        galley,
        fg,
    );
    resp
}

/// Muted field label above an input.
pub fn field_label(ui: &mut Ui, text: &str) {
    ui.label(RichText::new(text).color(theme::TEXT_MUTED).size(12.5));
}

/// Table header cell.
pub fn th(ui: &mut Ui, text: &str) {
    ui.label(
        RichText::new(text)
            .font(theme::semibold(12.5))
            .color(theme::TEXT_MUTED),
    );
}

/// A long key shown in monospace, wrapped, selectable.
pub fn key_box(ui: &mut Ui, key: &str) {
    Frame::none()
        .fill(theme::HEADER_BG)
        .rounding(Rounding::same(5.0))
        .inner_margin(Margin::same(10.0))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.add(
                egui::Label::new(
                    RichText::new(key)
                        .font(theme::mono(12.0))
                        .color(theme::TEXT),
                )
                .wrap()
                .selectable(true),
            );
        });
}
