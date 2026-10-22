//! The dashboard window: the Schedulo header, one card per product with
//! its tile, version, update state and an Open button.

use crate::brand;
use crate::catalogue::{Product, PRODUCTS};
use crate::icons::Icon;
use crate::install::{self, Folders};
use crate::theme;
use crate::ui::{self, Kind};
use crate::updater::{self, Event, State};
use eframe::egui::{self, Align, Frame, Layout, Margin, RichText, Sense, Ui, Vec2};
use std::collections::BTreeMap;
use std::sync::mpsc::{channel, Receiver};

pub struct App {
    folders: Result<Folders, String>,
    states: BTreeMap<&'static str, State>,
    rx: Option<Receiver<Event>>,
    offline: Option<String>,
    revoked: Option<usize>,
    done: bool,
    message: Option<(String, bool)>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> App {
        theme::install_fonts(&cc.egui_ctx);
        ui::apply_style(&cc.egui_ctx);
        let folders = match (install::local_root(), install::shared_root()) {
            (Some(l), Some(s)) => Folders::create(&l, &s),
            _ => Err("this computer has no app data folder".into()),
        };
        let mut app = App {
            folders,
            states: PRODUCTS
                .iter()
                .map(|p| (p.short, State::Checking))
                .collect(),
            rx: None,
            offline: None,
            revoked: None,
            done: false,
            message: None,
        };
        app.start(&cc.egui_ctx);
        app
    }

    /// Start the update check in the background.
    fn start(&mut self, ctx: &egui::Context) {
        let Ok(folders) = self.folders.clone() else {
            self.done = true;
            return;
        };
        for s in self.states.values_mut() {
            *s = State::Checking;
        }
        self.offline = None;
        self.done = false;
        let (tx, rx) = channel();
        let ctx = ctx.clone();
        let started = std::thread::Builder::new()
            .name("updater".into())
            .spawn(move || updater::run(folders, tx, move || ctx.request_repaint()));
        if started.is_ok() {
            self.rx = Some(rx);
        } else {
            self.done = true;
        }
    }

    fn poll(&mut self) {
        let Some(rx) = &self.rx else { return };
        while let Ok(e) = rx.try_recv() {
            match e {
                Event::Product(short, s) => {
                    self.states.insert(short, s);
                }
                Event::Offline(e) => {
                    self.offline = Some(e);
                    // Show what is installed instead.
                    if let Ok(f) = &self.folders {
                        let installed = f.installed();
                        for p in PRODUCTS {
                            self.states.insert(
                                p.short,
                                match installed.get(p.short) {
                                    Some(v) if f.program(p.short).is_some() => {
                                        State::UpToDate(v.clone())
                                    }
                                    _ => State::NotAvailable,
                                },
                            );
                        }
                    }
                }
                Event::Revoked(n) => self.revoked = Some(n),
                Event::Done => self.done = true,
            }
        }
        if self.done {
            self.rx = None;
        }
    }

    fn open(&mut self, p: &Product) {
        let Ok(f) = &self.folders else { return };
        let Some(exe) = f.program(p.short) else {
            self.message = Some((format!("{} isn't installed yet.", p.name), true));
            return;
        };
        let mut cmd = std::process::Command::new(&exe);
        if let Some(dir) = exe.parent() {
            cmd.current_dir(dir);
        }
        match cmd.spawn() {
            Ok(_) => self.message = Some((format!("Opening {}…", p.name), false)),
            Err(e) => self.message = Some((format!("Could not open {}: {e}", p.name), true)),
        }
    }

    fn header(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("header")
            .exact_height(66.0)
            .frame(
                Frame::none()
                    .fill(theme::NAV_BG)
                    .inner_margin(Margin::symmetric(20.0, 8.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    brand::master_lockup(ui);
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let busy = self.rx.is_some();
                        if ui::button_ex(ui, Some(Icon::Repeat), "Check again", Kind::Normal, !busy)
                            .clicked()
                        {
                            self.start(ctx);
                        }
                        let (text, colour) = if busy {
                            ("Checking for updates…".to_string(), theme::NAV_TEXT_DIM)
                        } else if self.offline.is_some() {
                            (
                                "Offline: using the installed versions".to_string(),
                                theme::NAV_TEXT_DIM,
                            )
                        } else {
                            ("Everything is up to date".to_string(), theme::NAV_TEXT)
                        };
                        if busy {
                            ui.spinner();
                        }
                        ui.label(RichText::new(text).size(12.5).color(colour));
                    });
                });
            });
    }

    fn card(&mut self, ui: &mut Ui, p: &Product, width: f32) {
        let state = self.states.get(p.short).cloned().unwrap_or(State::Checking);
        let installed = self
            .folders
            .as_ref()
            .ok()
            .and_then(|f| f.program(p.short))
            .is_some();
        let mut open = false;
        // Each card lays itself out top to bottom inside a fixed-size slot.
        ui.allocate_ui_with_layout(
            Vec2::new(width, 142.0),
            Layout::top_down(Align::Min),
            |ui| {
                Frame::none()
                    .fill(theme::CARD)
                    .stroke(theme::stroke(1.0, theme::BORDER))
                    .rounding(10.0)
                    .inner_margin(Margin::same(14.0))
                    .show(ui, |ui| {
                        ui.set_width(width - 28.0);
                        ui.set_height(112.0);
                        // Accent bar along the top.
                        let (bar, _) = ui.allocate_exact_size(
                            Vec2::new(ui.available_width(), 4.0),
                            Sense::hover(),
                        );
                        ui.painter().rect_filled(bar, 2.0, brand::accent(p));
                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            let (rect, _) =
                                ui.allocate_exact_size(Vec2::splat(52.0), Sense::hover());
                            brand::paint_tile(ui.painter(), rect, p, !installed);
                            ui.add_space(6.0);
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing.y = 1.0;
                                ui.label(
                                    RichText::new("SCHEDULO")
                                        .font(theme::logo_font(11.0))
                                        .color(theme::BRAND_NAVY),
                                );
                                ui.label(
                                    RichText::new(
                                        p.name.trim_start_matches("Schedulo ").to_uppercase(),
                                    )
                                    .font(theme::logo_font(15.0))
                                    .color(theme::BRAND_NAVY),
                                );
                                ui.label(
                                    RichText::new(p.descriptor.to_uppercase())
                                        .font(theme::medium(9.0))
                                        .color(theme::TEXT_MUTED),
                                );
                            });
                        });
                        ui.add_space(10.0);
                        let (badge, fg, bg) = match &state {
                            State::Checking => {
                                ("Checking…".to_string(), theme::TEXT_MUTED, theme::HEADER_BG)
                            }
                            State::NotAvailable => (
                                "Coming soon".to_string(),
                                theme::TEXT_MUTED,
                                theme::HEADER_BG,
                            ),
                            State::Downloading(v) => {
                                (format!("Updating to v{v}…"), theme::INFO, theme::INFO_SOFT)
                            }
                            State::UpToDate(v) => (
                                format!("v{v} · up to date"),
                                theme::SUCCESS,
                                theme::SUCCESS_SOFT,
                            ),
                            State::Updated(v) => (
                                format!("Updated to v{v}"),
                                theme::SUCCESS,
                                theme::SUCCESS_SOFT,
                            ),
                            State::Failed(_) => (
                                "Update failed".to_string(),
                                theme::WARNING,
                                theme::WARNING_SOFT,
                            ),
                        };
                        ui.horizontal(|ui| {
                            let r = ui::badge(ui, &badge, fg, bg);
                            if let State::Failed(e) = &state {
                                r.on_hover_text(format!("{e}. The installed version still works."));
                            }
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                if ui::button_ex(
                                    ui,
                                    Some(Icon::Open),
                                    "Open",
                                    Kind::Primary,
                                    installed,
                                )
                                .on_disabled_hover_text("Not installed yet")
                                .clicked()
                                {
                                    open = true;
                                }
                            });
                        });
                    });
            },
        );
        if open {
            self.open(p);
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll();
        self.header(ctx);
        egui::TopBottomPanel::bottom("footer")
            .exact_height(34.0)
            .frame(
                Frame::none()
                    .fill(theme::BG)
                    .inner_margin(Margin::symmetric(24.0, 6.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    brand::pillars(ui, theme::TEXT_FAINT);
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(theme::faint(concat!(
                            "Schedulo v",
                            env!("CARGO_PKG_VERSION"),
                            " · Newey Transport Services"
                        )));
                        if let Some(n) = self.revoked {
                            ui.label(theme::faint(format!(
                                "Licence check list updated ({n} revoked key{}) ·",
                                if n == 1 { "" } else { "s" }
                            )));
                        }
                    });
                });
            });
        egui::CentralPanel::default()
            .frame(
                Frame::none()
                    .fill(theme::BG)
                    .inner_margin(Margin::same(24.0)),
            )
            .show(ctx, |ui| {
                ui.label(
                    RichText::new("Your Schedulo apps")
                        .font(theme::semibold(22.0))
                        .color(theme::TEXT),
                );
                ui.label(theme::muted(
                    "Schedulo checks for new versions every time it opens and installs them for you.",
                ));
                if let Err(e) = &self.folders {
                    ui.add_space(6.0);
                    ui::error(ui, format!("Schedulo can't install apps: {e}."));
                }
                if let Some((m, err)) = &self.message {
                    ui.add_space(6.0);
                    if *err {
                        ui::warning(ui, m.as_str());
                    } else {
                        ui::info(ui, m.as_str());
                    }
                }
                ui.add_space(14.0);
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let gap = 14.0;
                    let cols = ((ui.available_width() + gap) / (260.0 + gap)).floor().max(1.0);
                    let w = ((ui.available_width() - gap * (cols - 1.0)) / cols).floor();
                    for row in PRODUCTS.chunks(cols as usize) {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = gap;
                            for p in row {
                                self.card(ui, p, w);
                            }
                        });
                        ui.add_space(gap);
                    }
                });
            });
    }
}
