//! Schedulo branding for the dashboard: the master lockup (symbol,
//! SCHEDULO, TRANSPORT PLANNING SOFTWARE) and each product's app tile in its
//! brand-book accent. See `docs/BRAND.md` in the product repositories.

use crate::catalogue::Product;
use crate::logo::{self, SymbolColours};
use crate::theme;
use eframe::egui::{
    text::LayoutJob, Color32, Painter, Pos2, Rect, Response, Rounding, Sense, TextFormat, Ui, Vec2,
};

pub fn accent(p: &Product) -> Color32 {
    let [r, g, b] = p.accent;
    Color32::from_rgb(r, g, b)
}

/// `c` mixed towards `to` by `t` (0 = `c`, 1 = `to`).
pub fn mix(c: Color32, to: Color32, t: f32) -> Color32 {
    let m = |a: u8, b: u8| (a as f32 + (b as f32 - a as f32) * t).round() as u8;
    Color32::from_rgb(m(c.r(), to.r()), m(c.g(), to.g()), m(c.b(), to.b()))
}

/// A product's app tile: the accent with a lighter upper half and the
/// symbol in white and pale tints, as on the identity board.
pub fn paint_tile(p: &Painter, rect: Rect, product: &Product, faded: bool) {
    let mut base = accent(product);
    if faded {
        base = mix(base, Color32::from_rgb(0xDC, 0xE1, 0xEE), 0.6);
    }
    let top = mix(base, Color32::WHITE, 0.14);
    let rounding = rect.width() * 0.22;
    p.rect_filled(rect, rounding, base);
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
    let w = Color32::WHITE;
    let pale = mix(base, w, 0.55);
    let colours = SymbolColours {
        bars: [
            (pale, Some(w)),
            (w, None),
            (pale, Some(theme::BRAND_NAVY)),
            (pale, Some(w)),
        ],
        ticks: [w, w, w, w],
    };
    logo::paint_symbol(p, rect.shrink(rect.width() * 0.2), &colours);
}

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

/// The master Schedulo lockup on Midnight Navy.
pub fn master_lockup(ui: &mut Ui) -> Response {
    let sym_h = 38.0;
    let sym_w = sym_h * logo::SYMBOL_W / logo::SYMBOL_H;
    let cap = 19.0;
    let p = ui.painter().clone();
    let wm_w = logo::wordmark_size(&p, cap).x;
    let tag = p.layout_job(spaced(
        "TRANSPORT PLANNING SOFTWARE",
        8.0,
        theme::NAV_TEXT_DIM,
        2.2,
    ));
    let text_w = wm_w.max(tag.size().x);
    let (rect, resp) =
        ui.allocate_exact_size(Vec2::new(2.0 + sym_w + 12.0 + text_w, 46.0), Sense::hover());
    if ui.is_rect_visible(rect) {
        let sym = Rect::from_min_size(
            Pos2::new(rect.left() + 2.0, rect.center().y - sym_h / 2.0),
            Vec2::new(sym_w, sym_h),
        );
        logo::paint_symbol(&p, sym, &logo::Product::Txc.symbol_on_dark());
        let x = sym.right() + 12.0;
        let wm = logo::paint_wordmark(
            &p,
            Pos2::new(x, rect.top() + 6.0),
            cap,
            Color32::WHITE,
            theme::BRAND_ELECTRIC,
        );
        p.galley(Pos2::new(x + 1.0, wm.bottom() + 6.0), tag, Color32::WHITE);
    }
    resp
}

/// "PLAN | OPERATE | MANAGE | COMPLY | GROW".
pub fn pillars(ui: &mut Ui, colour: Color32) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 10.0;
        for (i, w) in ["PLAN", "OPERATE", "MANAGE", "COMPLY", "GROW"]
            .into_iter()
            .enumerate()
        {
            if i > 0 {
                ui.label(eframe::egui::RichText::new("|").color(theme::BRAND_ELECTRIC));
            }
            ui.label(spaced(w, 8.5, colour, 2.4));
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixing() {
        assert_eq!(mix(Color32::BLACK, Color32::WHITE, 0.5).r(), 128);
        assert_eq!(
            accent(&crate::catalogue::PRODUCTS[0]),
            theme::BRAND_ELECTRIC
        );
    }
}
