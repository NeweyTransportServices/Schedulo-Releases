//! The start-up check, on a background thread: read `releases.json`,
//! install anything newer, refresh the revoked-keys list. It reports each
//! step to the window; any failure is reported quietly and never stops the
//! dashboard from opening.

use crate::catalogue::{self, PRODUCTS};
use crate::install::Folders;
use std::sync::mpsc::Sender;
use std::time::Duration;

/// What is happening to one product.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum State {
    Checking,
    /// Not released yet.
    NotAvailable,
    Downloading(String),
    /// Installed and current.
    UpToDate(String),
    /// Just updated to this version.
    Updated(String),
    /// Couldn't update; the reason (the installed version still works).
    Failed(String),
}

/// A message from the updater.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Product(&'static str, State),
    /// `releases.json` could not be read (offline?).
    Offline(String),
    /// A newer revoked-keys list was saved: this many keys.
    Revoked(usize),
    Done,
}

/// Run the whole check, sending events to `tx`.
pub fn run(folders: Folders, tx: Sender<Event>, wake: impl Fn()) {
    let send = |e: Event| {
        let _ = tx.send(e);
        wake();
    };
    let agent = ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_secs(5))
        .timeout(Duration::from_secs(300))
        .try_proxy_from_env(true)
        .user_agent(concat!("Schedulo/", env!("CARGO_PKG_VERSION")))
        .build();

    match catalogue::fetch(&agent) {
        Err(e) => send(Event::Offline(e)),
        Ok(published) => {
            let installed = folders.installed();
            for p in PRODUCTS {
                let have = installed.get(p.short).cloned();
                let Some(entry) = published.get(p.short) else {
                    send(Event::Product(
                        p.short,
                        match have {
                            Some(v) => State::UpToDate(v),
                            None => State::NotAvailable,
                        },
                    ));
                    continue;
                };
                let version = entry.version.clone().unwrap_or_default();
                let Some(file) = entry.file.clone() else {
                    send(Event::Product(p.short, State::NotAvailable));
                    continue;
                };
                let program_missing = folders.program(p.short).is_none();
                if !catalogue::is_newer(&version, have.as_deref()) && !program_missing {
                    send(Event::Product(
                        p.short,
                        State::UpToDate(have.unwrap_or(version)),
                    ));
                    continue;
                }
                send(Event::Product(p.short, State::Downloading(version.clone())));
                let name = file.rsplit('/').next().unwrap_or(&file).to_string();
                let state = match catalogue::open(&agent, &file)
                    .and_then(|d| folders.install(d, p.short, &name, &version))
                {
                    Ok(()) => State::Updated(version),
                    Err(e) => State::Failed(e),
                };
                send(Event::Product(p.short, state));
            }
        }
    }

    if let Some(n) = crate::revoked::update(&agent, &folders.revoked_file) {
        send(Event::Revoked(n));
    }
    send(Event::Done);
}
