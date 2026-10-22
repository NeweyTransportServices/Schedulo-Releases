//! Schedulo — the dashboard for the Schedulo family. On every start it
//! creates the folders it needs, checks Schedulo-Releases for newer versions
//! of each app, downloads and installs them, refreshes the signed
//! revoked-keys list, and lets you open any installed app. No account or
//! access token is needed: everything it reads is public, and nothing it
//! reads contains source code.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// The look, logo and icons are the same as in the other Schedulo apps.
#[allow(dead_code)]
mod icons;
#[allow(dead_code)]
mod logo;
#[allow(dead_code)]
mod theme;
#[allow(dead_code)]
mod ui;

mod app;
mod brand;
mod catalogue;
mod install;
mod revoked;
mod updater;

use eframe::egui;
use std::sync::Arc;

const ICON_RGBA: &[u8] = include_bytes!("../assets/icon-64.rgba");

fn main() -> eframe::Result<()> {
    let mut viewport = egui::ViewportBuilder::default()
        .with_title("Schedulo")
        .with_app_id("schedulo")
        .with_inner_size([1200.0, 800.0])
        .with_min_inner_size([900.0, 600.0]);
    if ICON_RGBA.len() == 64 * 64 * 4 {
        viewport = viewport.with_icon(Arc::new(egui::IconData {
            rgba: ICON_RGBA.to_vec(),
            width: 64,
            height: 64,
        }));
    }
    let options = eframe::NativeOptions {
        viewport,
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "Schedulo",
        options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
}
