//! Colours, typography and egui style for the application.
//!
//! Branding: the Schedulo brand (owner's brand book and identity board in
//! `docs/brand/`, rules in `docs/BRAND.md`). Midnight Navy `#071433` for the
//! navigation, Electric Blue `#1E4BFF` as the Schedulo TXC product accent
//! (product labels, active navigation, selected schedule blocks), Light Blue
//! `#E5E9F7` for selections, Bright Blue `#3F7BFF` and Cyan `#00C2FF` for
//! secondary highlights, and Montserrat (bundled in `assets/fonts`, OFL) for
//! all interface text. Derived shades below are mixes of these tokens; add
//! new colours here rather than as literals in views.

use eframe::egui::{
    self, Color32, FontData, FontDefinitions, FontFamily, FontId, FontTweak, Margin, Rounding,
    Shadow, Stroke, TextStyle, Vec2,
};

// --- Brand master palette (docs/BRAND.md) -----------------------------------
/// Midnight Navy (primary).
pub const BRAND_NAVY: Color32 = Color32::from_rgb(0x07, 0x14, 0x33);
/// Electric Blue (primary); the Schedulo TXC product accent.
pub const BRAND_ELECTRIC: Color32 = Color32::from_rgb(0x1E, 0x4B, 0xFF);
/// Bright Blue.
pub const BRAND_BRIGHT: Color32 = Color32::from_rgb(0x3F, 0x7B, 0xFF);
/// Cyan.
#[allow(dead_code)] // brand token, kept with the palette
pub const BRAND_CYAN: Color32 = Color32::from_rgb(0x00, 0xC2, 0xFF);
/// Light Blue.
pub const BRAND_LIGHT: Color32 = Color32::from_rgb(0xE5, 0xE9, 0xF7);
/// Sky: Bright Blue and Cyan mixed half and half (the light segment of the
/// symbol's top bar).
pub const BRAND_SKY: Color32 = Color32::from_rgb(0x20, 0x9F, 0xFF);
/// Schedulo Licence Manager product accent (orange).
pub const BRAND_LICENCE: Color32 = Color32::from_rgb(0xFF, 0x88, 0x00);

// --- Navigation (dark, Midnight Navy) ---------------------------------------
pub const NAV_BG: Color32 = BRAND_NAVY;
pub const NAV_BG_RAISED: Color32 = Color32::from_rgb(0x0D, 0x1C, 0x40);
pub const NAV_HOVER: Color32 = Color32::from_rgb(0x13, 0x24, 0x4D);
pub const NAV_SELECTED: Color32 = Color32::from_rgb(0x16, 0x2C, 0x66);
pub const NAV_BORDER: Color32 = Color32::from_rgb(0x1C, 0x2C, 0x55);
pub const NAV_TEXT: Color32 = Color32::from_rgb(0xDC, 0xE2, 0xF2);
pub const NAV_TEXT_DIM: Color32 = Color32::from_rgb(0x9A, 0xA6, 0xC6);
pub const NAV_HEADING: Color32 = Color32::from_rgb(0x75, 0x81, 0xA6);
/// Icon of the active navigation item (a light Electric/Bright Blue).
pub const NAV_ICON_SELECTED: Color32 = Color32::from_rgb(0x7F, 0xA5, 0xFF);

// --- Workspace (light) ------------------------------------------------------
pub const BG: Color32 = Color32::from_rgb(0xF4, 0xF6, 0xFB);
pub const CARD: Color32 = Color32::WHITE;
pub const BORDER: Color32 = Color32::from_rgb(0xDC, 0xE1, 0xEE);
pub const BORDER_STRONG: Color32 = Color32::from_rgb(0xC2, 0xCA, 0xDE);
pub const TEXT: Color32 = BRAND_NAVY;
pub const TEXT_MUTED: Color32 = Color32::from_rgb(0x4A, 0x56, 0x77);
pub const TEXT_FAINT: Color32 = Color32::from_rgb(0x6B, 0x75, 0x91);
pub const ZEBRA: Color32 = Color32::from_rgb(0xF7, 0xF8, 0xFC);
pub const ROW_HOVER: Color32 = Color32::from_rgb(0xED, 0xF0, 0xFA);
pub const ROW_SELECTED: Color32 = Color32::from_rgb(0xD5, 0xDE, 0xFB);
pub const HEADER_BG: Color32 = Color32::from_rgb(0xEC, 0xEF, 0xF8);

pub const ACCENT: Color32 = BRAND_ELECTRIC;
pub const ACCENT_HOVER: Color32 = Color32::from_rgb(0x1A, 0x40, 0xE0);
pub const ACCENT_DARK: Color32 = Color32::from_rgb(0x13, 0x35, 0xC0);
pub const ACCENT_SOFT: Color32 = BRAND_LIGHT;
/// Fill of a disabled primary button.
pub const ACCENT_DISABLED: Color32 = Color32::from_rgb(0xA6, 0xB7, 0xFA);

// Semantic colours: AA contrast (4.5:1) as text on white and on their soft
// backgrounds (see docs/BRAND.md).
pub const SUCCESS: Color32 = Color32::from_rgb(0x18, 0x73, 0x48);
pub const SUCCESS_SOFT: Color32 = Color32::from_rgb(0xE1, 0xF4, 0xEA);
pub const WARNING: Color32 = Color32::from_rgb(0x9A, 0x5B, 0x08);
pub const WARNING_SOFT: Color32 = Color32::from_rgb(0xFD, 0xF1, 0xD8);
pub const ERROR: Color32 = Color32::from_rgb(0xC0, 0x36, 0x36);
pub const ERROR_SOFT: Color32 = Color32::from_rgb(0xFB, 0xE6, 0xE6);
pub const INFO: Color32 = Color32::from_rgb(0x00, 0x68, 0xB0);
pub const INFO_SOFT: Color32 = Color32::from_rgb(0xE0, 0xF5, 0xFF);

/// Map layer: dead runs (positioning links), drawn dashed.
pub const MAP_DEAD_RUN: Color32 = Color32::from_rgb(0x6B, 0x75, 0x91);
/// Map layer: garage markers.
pub const MAP_GARAGE: Color32 = BRAND_NAVY;

pub const ROUNDING: f32 = 6.0;
pub const SPACE: f32 = 8.0;

pub const SEMIBOLD: &str = "semibold";
pub const MEDIUM: &str = "medium";
pub const BOLD: &str = "bold";
/// Montserrat Black (capitals only): the SCHEDULO wordmark and product label.
pub const LOGO: &str = "logo";

/// A stroke of `width` points (helper avoiding float-literal inference issues).
pub fn stroke(width: f32, color: Color32) -> Stroke {
    Stroke { width, color }
}

pub fn semibold_family() -> FontFamily {
    FontFamily::Name(SEMIBOLD.into())
}

pub fn semibold(size: f32) -> FontId {
    FontId::new(size, semibold_family())
}

pub fn medium(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(MEDIUM.into()))
}

#[allow(dead_code)] // brand weight, available to views
pub fn bold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(BOLD.into()))
}

/// The heavy wordmark face (capital letters, space, `&` and `|` only).
pub fn logo_font(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(LOGO.into()))
}

pub fn regular(size: f32) -> FontId {
    FontId::new(size, FontFamily::Proportional)
}

pub fn mono(size: f32) -> FontId {
    FontId::new(size, FontFamily::Monospace)
}

/// Rich text helpers.
pub fn strong(text: impl Into<String>) -> egui::RichText {
    egui::RichText::new(text.into()).family(semibold_family())
}

pub fn muted(text: impl Into<String>) -> egui::RichText {
    egui::RichText::new(text.into()).color(TEXT_MUTED)
}

pub fn faint(text: impl Into<String>) -> egui::RichText {
    egui::RichText::new(text.into())
        .color(TEXT_FAINT)
        .size(12.0)
}

/// Montserrat, bundled (SIL Open Font License, `assets/fonts/OFL.txt`),
/// subset to Latin. Weights per the brand book: Regular, Medium, SemiBold and
/// Bold; Black only for the wordmark.
const MONTSERRAT: [(&str, &[u8]); 5] = [
    (
        "montserrat-regular",
        include_bytes!("../assets/fonts/Montserrat-Regular.ttf"),
    ),
    (
        "montserrat-medium",
        include_bytes!("../assets/fonts/Montserrat-Medium.ttf"),
    ),
    (
        "montserrat-semibold",
        include_bytes!("../assets/fonts/Montserrat-SemiBold.ttf"),
    ),
    (
        "montserrat-bold",
        include_bytes!("../assets/fonts/Montserrat-Bold.ttf"),
    ),
    (
        "montserrat-black-logo",
        include_bytes!("../assets/fonts/Montserrat-Black-Logo.ttf"),
    ),
];

/// Montserrat is wider than the Segoe UI the layout was designed with; draw
/// it slightly smaller so the same point sizes fit the same space.
const MONTSERRAT_SCALE: f32 = 0.93;

/// Try to read the first file that exists from a list of candidates.
fn read_first(candidates: &[&str]) -> Option<Vec<u8>> {
    candidates.iter().find_map(|p| std::fs::read(p).ok())
}

/// Install fonts: Montserrat (bundled) for all interface text, then a system
/// sans (Segoe UI on Windows) and egui's bundled fonts as fallbacks for
/// symbols Montserrat does not have.
pub fn install_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    for (name, bytes) in MONTSERRAT {
        // The wordmark face keeps its true size (the logo is measured in it).
        let scale = if name.ends_with("-logo") {
            1.0
        } else {
            MONTSERRAT_SCALE
        };
        fonts.font_data.insert(
            name.into(),
            FontData::from_static(bytes).tweak(FontTweak {
                scale,
                ..Default::default()
            }),
        );
    }

    let windir = std::env::var("WINDIR").unwrap_or_else(|_| "C:\\Windows".to_string());
    let seg_regular = format!("{windir}\\Fonts\\segoeui.ttf");
    let fallback = read_first(&[
        seg_regular.as_str(),
        "/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
    ]);
    // egui's own fonts (symbols, emoji) come last in every family.
    let mut tail: Vec<String> = Vec::new();
    if let Some(bytes) = fallback {
        fonts
            .font_data
            .insert("ui-fallback".into(), FontData::from_owned(bytes));
        tail.push("ui-fallback".into());
    }
    tail.extend(
        fonts
            .families
            .get(&FontFamily::Proportional)
            .cloned()
            .unwrap_or_default(),
    );
    let family = |first: &[&str]| -> Vec<String> {
        first
            .iter()
            .map(|s| s.to_string())
            .chain(tail.iter().cloned())
            .collect()
    };
    fonts
        .families
        .insert(FontFamily::Proportional, family(&["montserrat-regular"]));
    fonts.families.insert(
        FontFamily::Name(MEDIUM.into()),
        family(&["montserrat-medium"]),
    );
    fonts
        .families
        .insert(semibold_family(), family(&["montserrat-semibold"]));
    fonts
        .families
        .insert(FontFamily::Name(BOLD.into()), family(&["montserrat-bold"]));
    fonts.families.insert(
        FontFamily::Name(LOGO.into()),
        family(&["montserrat-black-logo", "montserrat-bold"]),
    );

    ctx.set_fonts(fonts);
}

/// Apply the light workspace style.
pub fn apply_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    style.text_styles = [
        (TextStyle::Small, regular(11.5)),
        (TextStyle::Body, regular(13.5)),
        (TextStyle::Button, regular(13.5)),
        (TextStyle::Heading, semibold(20.0)),
        (TextStyle::Monospace, mono(12.5)),
    ]
    .into();

    let s = &mut style.spacing;
    s.item_spacing = Vec2::new(SPACE, 6.0);
    s.button_padding = Vec2::new(10.0, 4.0);
    s.interact_size = Vec2::new(36.0, 26.0);
    s.window_margin = Margin::same(16.0);
    s.menu_margin = Margin::same(6.0);
    s.combo_width = 160.0;
    s.text_edit_width = 220.0;
    s.icon_width = 16.0;
    s.icon_width_inner = 9.0;
    s.icon_spacing = 6.0;
    s.scroll = egui::style::ScrollStyle::solid();
    s.scroll.bar_width = 10.0;
    s.tooltip_width = 420.0;

    let v = &mut style.visuals;
    *v = egui::Visuals::light();
    v.override_text_color = None;
    v.panel_fill = BG;
    v.window_fill = CARD;
    v.window_stroke = crate::theme::stroke(1.0, BORDER);
    v.window_rounding = Rounding::same(8.0);
    v.menu_rounding = Rounding::same(6.0);
    v.window_shadow = Shadow {
        offset: Vec2::new(0.0, 6.0),
        blur: 24.0,
        spread: 0.0,
        color: Color32::from_black_alpha(40),
    };
    v.popup_shadow = Shadow {
        offset: Vec2::new(0.0, 4.0),
        blur: 14.0,
        spread: 0.0,
        color: Color32::from_black_alpha(30),
    };
    v.faint_bg_color = ZEBRA;
    v.extreme_bg_color = Color32::WHITE;
    v.code_bg_color = HEADER_BG;
    v.hyperlink_color = ACCENT;
    v.warn_fg_color = WARNING;
    v.error_fg_color = ERROR;
    v.selection.bg_fill = ROW_SELECTED;
    v.selection.stroke = crate::theme::stroke(1.0, ACCENT_DARK);
    v.striped = true;
    v.indent_has_left_vline = false;
    v.handle_shape = egui::style::HandleShape::Circle;
    v.slider_trailing_fill = true;

    let r = Rounding::same(5.0);
    let w = &mut v.widgets;
    w.noninteractive.bg_fill = CARD;
    w.noninteractive.weak_bg_fill = CARD;
    w.noninteractive.bg_stroke = crate::theme::stroke(1.0, BORDER);
    w.noninteractive.fg_stroke = crate::theme::stroke(1.0, TEXT);
    w.noninteractive.rounding = r;

    w.inactive.bg_fill = Color32::from_rgb(0xE4, 0xE8, 0xF3);
    w.inactive.weak_bg_fill = CARD;
    w.inactive.bg_stroke = crate::theme::stroke(1.0, BORDER_STRONG);
    w.inactive.fg_stroke = crate::theme::stroke(1.0, TEXT);
    w.inactive.rounding = r;
    w.inactive.expansion = 0.0;

    w.hovered.bg_fill = ROW_HOVER;
    w.hovered.weak_bg_fill = Color32::from_rgb(0xEF, 0xF2, 0xFA);
    w.hovered.bg_stroke = crate::theme::stroke(1.0, Color32::from_rgb(0xA4, 0xB0, 0xCC));
    w.hovered.fg_stroke = crate::theme::stroke(1.5, TEXT);
    w.hovered.rounding = r;
    w.hovered.expansion = 0.0;

    w.active.bg_fill = Color32::from_rgb(0xD9, 0xE0, 0xF4);
    w.active.weak_bg_fill = Color32::from_rgb(0xE1, 0xE7, 0xF6);
    w.active.bg_stroke = crate::theme::stroke(1.0, ACCENT);
    w.active.fg_stroke = crate::theme::stroke(1.5, TEXT);
    w.active.rounding = r;
    w.active.expansion = 0.0;

    w.open.bg_fill = Color32::from_rgb(0xE8, 0xEC, 0xF7);
    w.open.weak_bg_fill = Color32::from_rgb(0xE8, 0xEC, 0xF7);
    w.open.bg_stroke = crate::theme::stroke(1.0, BORDER_STRONG);
    w.open.fg_stroke = crate::theme::stroke(1.0, TEXT);
    w.open.rounding = r;

    style.interaction.tooltip_delay = 0.35;
    style.animation_time = 0.08;

    ctx.set_style(style);
}

/// Parse a TransXChange `LineColour` (`RRGGBB`, optional `#`).
pub fn parse_hex_colour(s: &str) -> Option<Color32> {
    let h = s.trim().trim_start_matches('#');
    if h.len() != 6 {
        return None;
    }
    let v = u32::from_str_radix(h, 16).ok()?;
    Some(Color32::from_rgb((v >> 16) as u8, (v >> 8) as u8, v as u8))
}

pub fn to_hex_colour(c: Color32) -> String {
    format!("{:02X}{:02X}{:02X}", c.r(), c.g(), c.b())
}

/// Palette used for lines without a declared colour.
pub const ROUTE_PALETTE: [Color32; 8] = [
    BRAND_ELECTRIC,
    Color32::from_rgb(0xD9, 0x5D, 0x1E),
    Color32::from_rgb(0x1C, 0x8C, 0x5E),
    Color32::from_rgb(0x8E, 0x44, 0xAD),
    Color32::from_rgb(0xC0, 0x36, 0x6B),
    Color32::from_rgb(0x00, 0x83, 0x9B),
    Color32::from_rgb(0x9C, 0x7A, 0x00),
    Color32::from_rgb(0x4A, 0x5A, 0x70),
];
