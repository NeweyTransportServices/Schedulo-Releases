//! The Schedulo logo, drawn as vectors with the egui painter: the
//! schedule-bar symbol, the SCHEDULO wordmark (Montserrat Black with the
//! segmented O) and the product label in the product accent.
//!
//! Geometry follows the owner's identity board (`docs/brand/`); the SVG files
//! in `assets/brand/` are drawn from the same numbers. See `docs/BRAND.md`.
//! Shared with the Licence Manager (`crates/nts-licence-manager`).

use crate::theme;
use eframe::egui::{
    epaint::PathShape, text::LayoutJob, Color32, Painter, Pos2, Rect, Response, Rounding, Sense,
    Stroke, TextFormat, Ui, Vec2,
};
use std::f32::consts::PI;

/// A Schedulo product: its label, descriptor and accent colour.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Product {
    Txc,
    /// Used by the Licence Manager (which shares this file).
    #[allow(dead_code)]
    LicenceManager,
}

impl Product {
    /// The product label under the wordmark, one entry per line.
    pub fn label(self) -> &'static [&'static str] {
        match self {
            Product::Txc => &["TXC"],
            Product::LicenceManager => &["LICENCE", "MANAGER"],
        }
    }

    /// The descriptor in spaced capitals.
    pub fn descriptor(self) -> &'static str {
        match self {
            Product::Txc => "TRANSXCHANGE",
            Product::LicenceManager => "COMPLIANCE & LICENSING",
        }
    }

    /// The product accent (brand book).
    pub fn accent(self) -> Color32 {
        match self {
            Product::Txc => theme::BRAND_ELECTRIC,
            Product::LicenceManager => theme::BRAND_LICENCE,
        }
    }

    /// The accent used for the product label on Midnight Navy. Electric Blue
    /// on navy is too dark to read (2.9:1), so the TXC label uses Bright Blue
    /// there; orange reads well on navy as it is.
    pub fn accent_on_dark(self) -> Color32 {
        match self {
            Product::Txc => theme::BRAND_BRIGHT,
            Product::LicenceManager => theme::BRAND_LICENCE,
        }
    }

    /// Colours of the symbol on a light background.
    pub fn symbol_on_light(self) -> SymbolColours {
        let navy = theme::BRAND_NAVY;
        let (strong, mid, light) = self.blues();
        SymbolColours {
            bars: [
                (light, Some(navy)),
                (strong, None),
                (mid, Some(navy)),
                (strong, Some(navy)),
            ],
            ticks: [navy, strong, mid, strong],
        }
    }

    /// Colours of the symbol on Midnight Navy (reversed: navy becomes white).
    pub fn symbol_on_dark(self) -> SymbolColours {
        let w = Color32::WHITE;
        let (strong, mid, light) = self.blues();
        SymbolColours {
            bars: [
                (light, Some(w)),
                (mid, None),
                (strong, Some(w)),
                (strong, Some(w)),
            ],
            ticks: [w, mid, w, mid],
        }
    }

    /// Colours of the symbol on the app-icon tile.
    pub fn symbol_on_tile(self) -> SymbolColours {
        match self {
            Product::Txc => self.symbol_on_dark(),
            Product::LicenceManager => {
                let w = Color32::WHITE;
                let pale = Color32::from_rgb(0xFF, 0xD5, 0x8A);
                let navy = theme::BRAND_NAVY;
                SymbolColours {
                    bars: [
                        (pale, Some(w)),
                        (pale, None),
                        (pale, Some(navy)),
                        (pale, Some(w)),
                    ],
                    ticks: [w, w, w, w],
                }
            }
        }
    }

    /// The app-icon tile colours (top, bottom of a vertical gradient).
    pub fn tile(self) -> (Color32, Color32) {
        match self {
            // The identity board's TXC tile: a deep blue between Midnight
            // Navy and Electric Blue.
            Product::Txc => (
                Color32::from_rgb(0x0E, 0x2A, 0x8E),
                Color32::from_rgb(0x12, 0x33, 0xB0),
            ),
            Product::LicenceManager => (
                Color32::from_rgb(0xFF, 0x9A, 0x1F),
                Color32::from_rgb(0xFF, 0x7A, 0x00),
            ),
        }
    }

    /// (strong, mid, light) tones of the product colour used in the bars.
    fn blues(self) -> (Color32, Color32, Color32) {
        match self {
            Product::Txc => (theme::BRAND_ELECTRIC, theme::BRAND_BRIGHT, theme::BRAND_SKY),
            Product::LicenceManager => (
                theme::BRAND_LICENCE,
                Color32::from_rgb(0xFF, 0x9F, 0x33),
                Color32::from_rgb(0xFF, 0xB5, 0x4D),
            ),
        }
    }
}

/// Colours of the four bars: (base, overlay segment) and the tick colours.
#[derive(Clone, Copy, Debug)]
pub struct SymbolColours {
    pub bars: [(Color32, Option<Color32>); 4],
    pub ticks: [Color32; 4],
}

/// The symbol's design box (units); bars are 20.4 units tall.
pub const SYMBOL_W: f32 = 100.0;
pub const SYMBOL_H: f32 = 95.4;

/// One schedule bar: pill x-range, top y, overlay x-range, tick side.
struct Bar {
    x0: f32,
    x1: f32,
    y: f32,
    seg: (f32, f32),
    tick_left: bool,
}

const BAR_H: f32 = 20.4;
const BAR_R: f32 = 9.0;
const TICK_W: f32 = 2.6;
const TICK_CAP: f32 = 14.0;
const BARS: [Bar; 4] = [
    Bar {
        x0: 10.0,
        x1: 94.4,
        y: 0.0,
        seg: (10.0, 50.0),
        tick_left: true,
    },
    Bar {
        x0: 0.0,
        x1: 75.6,
        y: 25.0,
        seg: (0.0, 0.0),
        tick_left: false,
    },
    Bar {
        x0: 9.6,
        x1: 92.0,
        y: 50.0,
        seg: (44.0, 92.0),
        tick_left: true,
    },
    Bar {
        x0: 16.0,
        x1: 91.4,
        y: 75.0,
        seg: (16.0, 58.0),
        tick_left: false,
    },
];

/// Paint the symbol, fitted and centred in `rect`.
pub fn paint_symbol(p: &Painter, rect: Rect, c: &SymbolColours) {
    let k = (rect.width() / SYMBOL_W).min(rect.height() / SYMBOL_H);
    let o = rect.center() - Vec2::new(SYMBOL_W, SYMBOL_H) * k / 2.0;
    let r = |x0: f32, y0: f32, x1: f32, y1: f32| {
        Rect::from_min_max(o + Vec2::new(x0, y0) * k, o + Vec2::new(x1, y1) * k)
    };
    for (i, b) in BARS.iter().enumerate() {
        let (base, seg) = c.bars[i];
        let tick = c.ticks[i];
        let cy = b.y + BAR_H / 2.0;
        // Tick: a stem from the bar end to a short vertical cap.
        let (sx0, sx1, capx) = if b.tick_left {
            (0.0, b.x0 + 1.0, 0.0)
        } else {
            (b.x1 - 1.0, SYMBOL_W, SYMBOL_W - TICK_W)
        };
        p.rect_filled(r(sx0, cy - TICK_W / 2.0, sx1, cy + TICK_W / 2.0), 0.0, tick);
        p.rect_filled(
            r(
                capx,
                cy - TICK_CAP / 2.0,
                capx + TICK_W,
                cy + TICK_CAP / 2.0,
            ),
            TICK_W * k / 2.0,
            tick,
        );
        p.rect_filled(r(b.x0, b.y, b.x1, b.y + BAR_H), BAR_R * k, base);
        if let Some(sc) = seg {
            p.rect_filled(r(b.seg.0, b.y, b.seg.1, b.y + BAR_H), BAR_R * k, sc);
        }
    }
}

/// The app icon: a rounded tile with the reversed symbol.
pub fn paint_app_icon(p: &Painter, rect: Rect, product: Product) {
    let (top, bottom) = product.tile();
    let rounding = rect.width() * 0.22;
    p.rect_filled(rect, rounding, bottom);
    // A soft top-to-bottom gradient: the upper part in the lighter tone.
    let upper = Rect::from_min_max(rect.min, Pos2::new(rect.max.x, rect.center().y));
    p.rect_filled(
        upper,
        Rounding {
            nw: rounding,
            ne: rounding,
            sw: 0.0,
            se: 0.0,
        },
        top,
    );
    let inner = rect.shrink(rect.width() * 0.2);
    paint_symbol(p, inner, &product.symbol_on_tile());
}

/// Cap height of the wordmark face as a fraction of its font size.
const CAP: f32 = 0.70;
/// Ascent above the cap line (Montserrat: ascent 0.968 em).
const ABOVE_CAP: f32 = 0.268;

/// The wordmark's size for a cap height.
pub fn wordmark_size(p: &Painter, cap: f32) -> Vec2 {
    let size = cap / CAP;
    let g = p.layout_no_wrap("SCHEDUL".into(), theme::logo_font(size), Color32::WHITE);
    Vec2::new(g.size().x + 0.848 * size, cap)
}

/// Paint SCHEDULO with its cap line at `top_left.y`; returns the rect.
/// `ink` is the letters' colour, `accent` the O's quarter segment (always
/// Electric Blue: the wordmark is the same in every product).
pub fn paint_wordmark(
    p: &Painter,
    top_left: Pos2,
    cap: f32,
    ink: Color32,
    accent: Color32,
) -> Rect {
    let size = cap / CAP;
    let g = p.layout_no_wrap("SCHEDUL".into(), theme::logo_font(size), ink);
    let w = g.size().x;
    p.galley(top_left - Vec2::new(0.0, ABOVE_CAP * size), g, ink);
    // The segmented O: a ring the size of the glyph (Montserrat Black O:
    // 790 x 732 units in an 848 advance), stroke about the stem weight.
    let r_out = 0.366 * size;
    let thick = 0.225 * size;
    let centre = Pos2::new(top_left.x + w + 0.424 * size, top_left.y + cap / 2.0);
    paint_ring(p, centre, r_out, thick, ink, accent);
    Rect::from_min_size(top_left, Vec2::new(w + 0.848 * size, cap))
}

/// The segmented ring: `base` all round except an `accent` quarter from 12
/// o'clock clockwise to just past 3 o'clock, with thin cuts between.
pub fn paint_ring(p: &Painter, c: Pos2, r_out: f32, thick: f32, base: Color32, accent: Color32) {
    let r = r_out - thick / 2.0;
    // Angles in degrees, clockwise from 12 o'clock.
    let gap = (0.10 * thick / r).to_degrees().max(4.0);
    arc(p, c, r, thick, gap / 2.0, 105.0 - gap / 2.0, accent);
    arc(p, c, r, thick, 105.0 + gap / 2.0, 360.0 - gap / 2.0, base);
}

fn arc(p: &Painter, c: Pos2, r: f32, thick: f32, from: f32, to: f32, colour: Color32) {
    let steps = (((to - from) / 4.0).ceil() as usize).max(2);
    let pts: Vec<Pos2> = (0..=steps)
        .map(|i| {
            let a = (from + (to - from) * i as f32 / steps as f32) * PI / 180.0;
            Pos2::new(c.x + r * a.sin(), c.y - r * a.cos())
        })
        .collect();
    p.add(PathShape::line(pts, Stroke::new(thick, colour)));
}

/// Spaced capitals (descriptor / tagline).
fn spaced(text: &str, size: f32, colour: Color32, spacing: f32) -> LayoutJob {
    let mut job = LayoutJob::default();
    job.append(
        text,
        0.0,
        TextFormat {
            font_id: theme::medium(size),
            color: colour,
            extra_letter_spacing: spacing,
            ..Default::default()
        },
    );
    job
}

/// The horizontal lockup on Midnight Navy (TXC sidebar, Licence Manager
/// header): symbol, then SCHEDULO with the product label and descriptor
/// beneath.
pub fn sidebar_lockup(ui: &mut Ui, product: Product) -> Response {
    let sym_h = 36.0;
    let sym_w = sym_h * SYMBOL_W / SYMBOL_H;
    let cap = 15.0;
    let label_cap = 11.0;
    let size = label_cap / CAP;
    let p = ui.painter().clone();
    let wm_w = wordmark_size(&p, cap).x;
    let g = p.layout_no_wrap(
        product.label().join(" "),
        theme::logo_font(size),
        product.accent_on_dark(),
    );
    let d = p.layout_job(spaced(product.descriptor(), 8.5, theme::NAV_TEXT_DIM, 1.6));
    let text_w = wm_w.max(g.size().x + 8.0 + d.size().x);
    let (rect, resp) =
        ui.allocate_exact_size(Vec2::new(2.0 + sym_w + 10.0 + text_w, 44.0), Sense::hover());
    if ui.is_rect_visible(rect) {
        let sym = Rect::from_min_size(
            Pos2::new(rect.left() + 2.0, rect.center().y - sym_h / 2.0),
            Vec2::new(sym_w, sym_h),
        );
        paint_symbol(&p, sym, &product.symbol_on_dark());
        let x = sym.right() + 10.0;
        let wm = paint_wordmark(
            &p,
            Pos2::new(x, rect.top() + 4.0),
            cap,
            Color32::WHITE,
            theme::BRAND_ELECTRIC,
        );
        let ly = wm.bottom() + 7.0;
        let lw = g.size().x;
        p.galley(Pos2::new(x, ly - ABOVE_CAP * size), g, Color32::WHITE);
        let dy = ly + label_cap / 2.0 - d.size().y / 2.0;
        p.galley(Pos2::new(x + lw + 8.0, dy), d, Color32::WHITE);
    }
    resp
}

/// The stacked product logo (identity board "sub brand" layout): symbol,
/// SCHEDULO, the product label in the accent and the descriptor, centred.
/// `cap` is the wordmark cap height; `on_dark` reverses the colours.
pub fn stacked_lockup(ui: &mut Ui, product: Product, cap: f32, on_dark: bool) -> Response {
    let wm = wordmark_size(ui.painter(), cap);
    let sym_h = cap * 3.0;
    let label_cap = cap * 0.82;
    let lines = product.label().len() as f32;
    let desc_size = (cap * 0.42).max(8.5);
    let h = sym_h
        + cap * 0.6
        + cap
        + cap * 0.45
        + lines * label_cap * 1.3
        + cap * 0.5
        + desc_size * 1.3;
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(ui.available_width(), h), Sense::hover());
    if ui.is_rect_visible(rect) {
        let p = ui.painter();
        let ink = if on_dark {
            Color32::WHITE
        } else {
            theme::BRAND_NAVY
        };
        let accent = if on_dark {
            product.accent_on_dark()
        } else {
            product.accent()
        };
        let cx = rect.center().x;
        let mut y = rect.top();
        let sym = Rect::from_center_size(
            Pos2::new(cx, y + sym_h / 2.0),
            Vec2::new(sym_h * SYMBOL_W / SYMBOL_H, sym_h),
        );
        let colours = if on_dark {
            product.symbol_on_dark()
        } else {
            product.symbol_on_light()
        };
        paint_symbol(p, sym, &colours);
        y += sym_h + cap * 0.6;
        paint_wordmark(
            p,
            Pos2::new(cx - wm.x / 2.0, y),
            cap,
            ink,
            theme::BRAND_ELECTRIC,
        );
        y += cap + cap * 0.45;
        let size = label_cap / CAP;
        for line in product.label() {
            let g = p.layout_no_wrap(line.to_string(), theme::logo_font(size), accent);
            p.galley(
                Pos2::new(cx - g.size().x / 2.0, y - ABOVE_CAP * size),
                g,
                accent,
            );
            y += label_cap * 1.3;
        }
        y += cap * 0.5 - label_cap * 0.3;
        let dc = if on_dark {
            theme::NAV_TEXT_DIM
        } else {
            theme::TEXT_MUTED
        };
        let d = p.layout_job(spaced(
            product.descriptor(),
            desc_size,
            dc,
            desc_size * 0.28,
        ));
        p.galley(Pos2::new(cx - d.size().x / 2.0, y), d, dc);
    }
    resp
}

#[cfg(test)]
mod tests {
    use super::*;
    use eframe::egui;

    #[test]
    fn bars_fit_the_design_box() {
        for b in &BARS {
            assert!(b.x0 >= 0.0 && b.x1 <= SYMBOL_W);
            assert!(b.y + BAR_H <= SYMBOL_H + 0.01);
        }
    }

    #[test]
    fn product_accents_follow_the_brand_book() {
        assert_eq!(Product::Txc.accent(), Color32::from_rgb(0x1E, 0x4B, 0xFF));
        assert_eq!(
            Product::LicenceManager.accent(),
            Color32::from_rgb(0xFF, 0x88, 0x00)
        );
        assert_eq!(Product::Txc.descriptor(), "TRANSXCHANGE");
    }

    #[test]
    fn paints_without_panicking() {
        let ctx = egui::Context::default();
        theme::install_fonts(&ctx);
        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                sidebar_lockup(ui, Product::Txc);
                stacked_lockup(ui, Product::LicenceManager, 20.0, false);
                paint_app_icon(
                    ui.painter(),
                    Rect::from_min_size(Pos2::ZERO, Vec2::splat(64.0)),
                    Product::Txc,
                );
            });
        });
    }
}
