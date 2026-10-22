//! Vector icons drawn with the egui painter on a 24×24 design grid.

use eframe::egui::{epaint::PathShape, Color32, Painter, Pos2, Rect, Response, Sense, Ui, Vec2};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icon {
    Open,
    New,
    Save,
    Overview,
    Services,
    Route,
    Journeys,
    Timetable,
    Stops,
    Map,
    Calendar,
    Validation,
    Export,
    Import,
    Download,
    Settings,
    Plus,
    Delete,
    Search,
    Warning,
    Error,
    Info,
    Check,
    Close,
    ChevronLeft,
    ChevronRight,
    ArrowUp,
    ArrowDown,
    Undo,
    Redo,
    Reverse,
    Clock,
    Copy,
    Repeat,
    Operator,
    Database,
    Bus,
    Crosshair,
    Key,
    Lock,
    People,
    Link,
}

/// Paint `icon` filling `rect` (square recommended).
pub fn paint(painter: &Painter, rect: Rect, icon: Icon, color: Color32) {
    let s = rect.width().min(rect.height()) / 24.0;
    let o = rect.center() - Vec2::splat(12.0 * s);
    let p = |x: f32, y: f32| Pos2::new(o.x + x * s, o.y + y * s);
    let w = (1.6 * s).max(1.15);
    let st = crate::theme::stroke(w, color);
    let line = |pts: &[(f32, f32)]| {
        let v: Vec<Pos2> = pts.iter().map(|(x, y)| p(*x, *y)).collect();
        painter.add(PathShape::line(v, st));
    };
    let closed = |pts: &[(f32, f32)]| {
        let v: Vec<Pos2> = pts.iter().map(|(x, y)| p(*x, *y)).collect();
        painter.add(PathShape::closed_line(v, st));
    };
    let rect_s = |x0: f32, y0: f32, x1: f32, y1: f32, r: f32| {
        painter.rect_stroke(Rect::from_min_max(p(x0, y0), p(x1, y1)), r * s, st);
    };
    let rect_f = |x0: f32, y0: f32, x1: f32, y1: f32, r: f32| {
        painter.rect_filled(Rect::from_min_max(p(x0, y0), p(x1, y1)), r * s, color);
    };
    let circle = |x: f32, y: f32, r: f32| painter.circle_stroke(p(x, y), r * s, st);
    let dot = |x: f32, y: f32, r: f32| painter.circle_filled(p(x, y), r * s, color);
    let arc = |cx: f32, cy: f32, r: f32, a0: f32, a1: f32| {
        let n = 16;
        let v: Vec<Pos2> = (0..=n)
            .map(|i| {
                let a = a0 + (a1 - a0) * i as f32 / n as f32;
                p(cx + r * a.cos(), cy + r * a.sin())
            })
            .collect();
        painter.add(PathShape::line(v, st));
    };

    use std::f32::consts::PI;
    match icon {
        Icon::Open => {
            closed(&[
                (3.0, 6.0),
                (9.5, 6.0),
                (11.5, 8.5),
                (21.0, 8.5),
                (21.0, 19.0),
                (3.0, 19.0),
            ]);
            line(&[(3.0, 11.5), (21.0, 11.5)]);
        }
        Icon::New => {
            closed(&[
                (5.5, 3.0),
                (14.0, 3.0),
                (19.0, 8.0),
                (19.0, 21.0),
                (5.5, 21.0),
            ]);
            line(&[(14.0, 3.0), (14.0, 8.0), (19.0, 8.0)]);
            line(&[(12.25, 11.5), (12.25, 17.5)]);
            line(&[(9.25, 14.5), (15.25, 14.5)]);
        }
        Icon::Save => {
            closed(&[
                (4.0, 4.0),
                (16.5, 4.0),
                (20.0, 7.5),
                (20.0, 20.0),
                (4.0, 20.0),
            ]);
            rect_s(8.0, 4.0, 15.0, 8.5, 0.5);
            rect_s(7.0, 13.0, 17.0, 20.0, 0.5);
        }
        Icon::Overview => {
            rect_s(4.0, 4.0, 11.0, 11.0, 1.5);
            rect_s(13.0, 4.0, 20.0, 8.5, 1.5);
            rect_s(13.0, 11.0, 20.0, 20.0, 1.5);
            rect_s(4.0, 13.5, 11.0, 20.0, 1.5);
        }
        Icon::Services => {
            rect_s(3.5, 4.5, 9.5, 9.5, 1.5);
            line(&[(12.5, 7.0), (20.5, 7.0)]);
            rect_s(3.5, 14.5, 9.5, 19.5, 1.5);
            line(&[(12.5, 17.0), (20.5, 17.0)]);
        }
        Icon::Route => {
            line(&[(5.5, 18.5), (5.5, 12.0), (18.5, 12.0), (18.5, 5.5)]);
            dot(5.5, 18.5, 2.4);
            painter.circle(
                p(12.0, 12.0),
                2.1 * s,
                Color32::WHITE.gamma_multiply(0.0),
                st,
            );
            dot(12.0, 12.0, 1.3);
            dot(18.5, 5.5, 2.4);
        }
        Icon::Journeys | Icon::Bus => {
            rect_s(5.0, 3.0, 19.0, 18.0, 2.5);
            rect_s(7.5, 6.0, 16.5, 11.0, 0.8);
            line(&[(8.0, 18.0), (8.0, 21.0)]);
            line(&[(16.0, 18.0), (16.0, 21.0)]);
            dot(8.6, 14.6, 1.1);
            dot(15.4, 14.6, 1.1);
        }
        Icon::Timetable => {
            rect_s(3.0, 4.5, 21.0, 19.5, 1.5);
            line(&[(3.0, 9.5), (21.0, 9.5)]);
            line(&[(3.0, 14.5), (21.0, 14.5)]);
            line(&[(9.0, 4.5), (9.0, 19.5)]);
            line(&[(15.0, 9.5), (15.0, 19.5)]);
        }
        Icon::Stops => {
            line(&[(8.0, 3.0), (8.0, 21.0)]);
            line(&[(5.0, 21.0), (11.0, 21.0)]);
            rect_s(8.0, 3.5, 18.0, 10.5, 1.5);
            dot(13.0, 7.0, 1.4);
        }
        Icon::Map => {
            closed(&[
                (3.0, 6.5),
                (9.0, 4.0),
                (15.0, 6.5),
                (21.0, 4.0),
                (21.0, 17.5),
                (15.0, 20.0),
                (9.0, 17.5),
                (3.0, 20.0),
            ]);
            line(&[(9.0, 4.0), (9.0, 17.5)]);
            line(&[(15.0, 6.5), (15.0, 20.0)]);
        }
        Icon::Calendar => {
            rect_s(3.5, 5.5, 20.5, 20.5, 2.0);
            line(&[(3.5, 10.0), (20.5, 10.0)]);
            line(&[(8.0, 3.0), (8.0, 7.5)]);
            line(&[(16.0, 3.0), (16.0, 7.5)]);
            for (x, y) in [
                (8.0, 14.0),
                (12.0, 14.0),
                (16.0, 14.0),
                (8.0, 17.5),
                (12.0, 17.5),
            ] {
                dot(x, y, 1.0);
            }
        }
        Icon::Validation => {
            closed(&[
                (12.0, 2.8),
                (19.5, 5.8),
                (19.5, 11.5),
                (17.0, 16.8),
                (12.0, 21.0),
                (7.0, 16.8),
                (4.5, 11.5),
                (4.5, 5.8),
            ]);
            line(&[(8.5, 12.0), (11.0, 14.5), (15.8, 9.3)]);
        }
        Icon::Export => {
            line(&[(4.0, 13.0), (4.0, 20.0), (20.0, 20.0), (20.0, 13.0)]);
            line(&[(12.0, 15.5), (12.0, 3.5)]);
            line(&[(7.5, 8.0), (12.0, 3.5), (16.5, 8.0)]);
        }
        Icon::Import | Icon::Download => {
            line(&[(4.0, 13.0), (4.0, 20.0), (20.0, 20.0), (20.0, 13.0)]);
            line(&[(12.0, 3.5), (12.0, 15.5)]);
            line(&[(7.5, 11.0), (12.0, 15.5), (16.5, 11.0)]);
        }
        Icon::Settings => {
            circle(12.0, 12.0, 3.0);
            circle(12.0, 12.0, 6.6);
            let tooth = crate::theme::stroke(w * 1.9, color);
            for i in 0..8 {
                let a = i as f32 * PI / 4.0;
                painter.line_segment(
                    [
                        p(12.0 + 6.8 * a.cos(), 12.0 + 6.8 * a.sin()),
                        p(12.0 + 9.2 * a.cos(), 12.0 + 9.2 * a.sin()),
                    ],
                    tooth,
                );
            }
        }
        Icon::Plus => {
            line(&[(12.0, 5.0), (12.0, 19.0)]);
            line(&[(5.0, 12.0), (19.0, 12.0)]);
        }
        Icon::Delete => {
            line(&[(4.0, 6.5), (20.0, 6.5)]);
            line(&[(9.0, 6.5), (9.0, 3.8), (15.0, 3.8), (15.0, 6.5)]);
            line(&[(6.0, 6.5), (7.0, 20.5), (17.0, 20.5), (18.0, 6.5)]);
            line(&[(10.0, 10.5), (10.0, 16.5)]);
            line(&[(14.0, 10.5), (14.0, 16.5)]);
        }
        Icon::Search => {
            circle(10.5, 10.5, 6.0);
            line(&[(15.0, 15.0), (20.0, 20.0)]);
        }
        Icon::Warning => {
            closed(&[(12.0, 3.0), (21.5, 19.5), (2.5, 19.5)]);
            line(&[(12.0, 9.0), (12.0, 13.8)]);
            dot(12.0, 16.6, 1.1);
        }
        Icon::Error => {
            circle(12.0, 12.0, 9.0);
            line(&[(8.8, 8.8), (15.2, 15.2)]);
            line(&[(15.2, 8.8), (8.8, 15.2)]);
        }
        Icon::Info => {
            circle(12.0, 12.0, 9.0);
            line(&[(12.0, 11.0), (12.0, 16.5)]);
            dot(12.0, 7.8, 1.15);
        }
        Icon::Check => line(&[(5.0, 12.5), (10.0, 17.5), (19.0, 7.0)]),
        Icon::Close => {
            line(&[(6.0, 6.0), (18.0, 18.0)]);
            line(&[(18.0, 6.0), (6.0, 18.0)]);
        }
        Icon::ChevronLeft => line(&[(15.0, 5.0), (8.0, 12.0), (15.0, 19.0)]),
        Icon::ChevronRight => line(&[(9.0, 5.0), (16.0, 12.0), (9.0, 19.0)]),
        Icon::ArrowUp => {
            line(&[(12.0, 20.0), (12.0, 4.5)]);
            line(&[(6.0, 10.5), (12.0, 4.5), (18.0, 10.5)]);
        }
        Icon::ArrowDown => {
            line(&[(12.0, 4.0), (12.0, 19.5)]);
            line(&[(6.0, 13.5), (12.0, 19.5), (18.0, 13.5)]);
        }
        Icon::Undo => {
            line(&[(5.0, 9.0), (14.5, 9.0)]);
            arc(14.5, 13.5, 4.5, -PI / 2.0, PI / 2.0);
            line(&[(14.5, 18.0), (8.0, 18.0)]);
            line(&[(8.5, 5.5), (5.0, 9.0), (8.5, 12.5)]);
        }
        Icon::Redo => {
            line(&[(19.0, 9.0), (9.5, 9.0)]);
            arc(9.5, 13.5, 4.5, -PI / 2.0, -PI * 1.5);
            line(&[(9.5, 18.0), (16.0, 18.0)]);
            line(&[(15.5, 5.5), (19.0, 9.0), (15.5, 12.5)]);
        }
        Icon::Reverse => {
            line(&[(4.0, 8.0), (19.5, 8.0)]);
            line(&[(16.0, 4.5), (19.5, 8.0), (16.0, 11.5)]);
            line(&[(20.0, 16.0), (4.5, 16.0)]);
            line(&[(8.0, 12.5), (4.5, 16.0), (8.0, 19.5)]);
        }
        Icon::Key => {
            circle(7.5, 15.5, 4.0);
            line(&[(10.4, 12.6), (20.0, 3.0)]);
            line(&[(16.5, 6.5), (19.0, 9.0)]);
            line(&[(14.0, 9.0), (16.0, 11.0)]);
        }
        Icon::Lock => {
            rect_s(5.0, 10.5, 19.0, 20.5, 2.0);
            arc(12.0, 10.5, 4.5, PI, 2.0 * PI);
            line(&[(7.5, 10.5), (7.5, 9.5)]);
            line(&[(16.5, 10.5), (16.5, 9.5)]);
            dot(12.0, 15.5, 1.5);
        }
        Icon::People => {
            // Two heads and shoulders, one behind the other.
            circle(9.0, 8.0, 3.2);
            arc(9.0, 19.5, 6.0, PI, 2.0 * PI);
            line(&[(3.0, 19.5), (15.0, 19.5)]);
            arc(16.5, 7.0, 2.6, 1.15 * PI, 2.6 * PI);
            arc(16.5, 17.5, 5.0, 1.35 * PI, 2.0 * PI);
            line(&[(21.5, 17.5), (17.0, 17.5)]);
        }
        Icon::Link => {
            rect_s(2.5, 8.5, 12.5, 15.5, 3.5);
            rect_s(11.5, 8.5, 21.5, 15.5, 3.5);
            line(&[(8.0, 12.0), (16.0, 12.0)]);
        }
        Icon::Clock => {
            circle(12.0, 12.0, 9.0);
            line(&[(12.0, 7.0), (12.0, 12.0), (15.5, 14.0)]);
        }
        Icon::Copy => {
            rect_s(8.5, 8.5, 20.0, 20.0, 1.8);
            line(&[
                (15.5, 8.5),
                (15.5, 4.0),
                (4.0, 4.0),
                (4.0, 15.5),
                (8.5, 15.5),
            ]);
        }
        Icon::Repeat => {
            line(&[(4.5, 11.0), (4.5, 7.0), (18.5, 7.0)]);
            line(&[(15.5, 4.0), (18.5, 7.0), (15.5, 10.0)]);
            line(&[(19.5, 13.0), (19.5, 17.0), (5.5, 17.0)]);
            line(&[(8.5, 14.0), (5.5, 17.0), (8.5, 20.0)]);
        }
        Icon::Operator => {
            rect_s(5.0, 4.0, 19.0, 20.5, 1.2);
            for (x, y) in [(9.0, 8.0), (15.0, 8.0), (9.0, 12.0), (15.0, 12.0)] {
                rect_f(x - 1.2, y - 1.0, x + 1.2, y + 1.0, 0.3);
            }
            rect_s(10.0, 16.0, 14.0, 20.5, 0.3);
        }
        Icon::Database => {
            let ell = |cy: f32, top_half: bool, full: bool| {
                let n = 24;
                let (a0, a1) = if full {
                    (0.0, 2.0 * PI)
                } else if top_half {
                    (PI, 2.0 * PI)
                } else {
                    (0.0, PI)
                };
                let v: Vec<Pos2> = (0..=n)
                    .map(|i| {
                        let a = a0 + (a1 - a0) * i as f32 / n as f32;
                        p(12.0 + 7.5 * a.cos(), cy + 2.8 * a.sin())
                    })
                    .collect();
                painter.add(PathShape::line(v, st));
            };
            ell(6.0, false, true);
            ell(12.0, false, false);
            ell(18.0, false, false);
            line(&[(4.5, 6.0), (4.5, 18.0)]);
            line(&[(19.5, 6.0), (19.5, 18.0)]);
        }
        Icon::Crosshair => {
            circle(12.0, 12.0, 6.5);
            line(&[(12.0, 2.5), (12.0, 7.0)]);
            line(&[(12.0, 17.0), (12.0, 21.5)]);
            line(&[(2.5, 12.0), (7.0, 12.0)]);
            line(&[(17.0, 12.0), (21.5, 12.0)]);
        }
    }
}

/// Allocate space and paint an icon (non-interactive).
pub fn icon(ui: &mut Ui, icon: Icon, size: f32, color: Color32) -> Response {
    let (rect, resp) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    if ui.is_rect_visible(rect) {
        paint(ui.painter(), rect, icon, color);
    }
    resp
}

/// Paint the Schedulo TXC app icon (rounded tile with the schedule-bar
/// symbol; see `logo.rs`).
pub fn paint_brand(painter: &Painter, rect: Rect) {
    crate::logo::paint_app_icon(painter, rect, crate::logo::Product::Txc);
}
