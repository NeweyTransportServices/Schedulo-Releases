//! The product list: which Schedulo apps exist, their brand colours, and
//! (from `releases.json` in this repository) the latest version of each.

use serde::Deserialize;
use std::collections::BTreeMap;
use std::io::Read;

/// Raw files of this repository.
pub const RAW_BASE: &str =
    "https://raw.githubusercontent.com/NeweyTransportServices/Schedulo-Releases/main/";

/// A Schedulo product as the brand book shows it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Product {
    /// Short label, as in `releases.json` and on the app icons.
    pub short: &'static str,
    pub name: &'static str,
    pub descriptor: &'static str,
    pub accent: [u8; 3],
}

/// Every product, in brand-book order. (The Licence Manager is never part
/// of Schedulo.)
pub const PRODUCTS: [Product; 10] = [
    Product {
        short: "TXC",
        name: "Schedulo TXC",
        descriptor: "TransXChange",
        accent: [0x1E, 0x4B, 0xFF],
    },
    Product {
        short: "BUS",
        name: "Schedulo Bus",
        descriptor: "Bus scheduling",
        accent: [0x00, 0xA6, 0xE8],
    },
    Product {
        short: "COACH",
        name: "Schedulo Coach",
        descriptor: "Coach scheduling",
        accent: [0xF2, 0x0D, 0x78],
    },
    Product {
        short: "COSTING",
        name: "Schedulo Costing",
        descriptor: "Operational costing",
        accent: [0x00, 0xB9, 0x6B],
    },
    Product {
        short: "NSA",
        name: "Schedulo NSA",
        descriptor: "Next Stop Announcements",
        accent: [0x70, 0x28, 0xF5],
    },
    Product {
        short: "FLEET",
        name: "Schedulo Fleet",
        descriptor: "Vehicle & asset management",
        accent: [0x00, 0xA9, 0x9D],
    },
    Product {
        short: "WORKSHOP",
        name: "Schedulo Workshop",
        descriptor: "Maintenance & workshop",
        accent: [0xF5, 0x22, 0x2D],
    },
    Product {
        short: "ROSTER",
        name: "Schedulo Roster",
        descriptor: "Crew rostering",
        accent: [0xF2, 0xB9, 0x00],
    },
    Product {
        short: "DRIVER",
        name: "Schedulo Driver",
        descriptor: "Driver management",
        accent: [0x8B, 0x5C, 0xF6],
    },
    Product {
        short: "TM",
        name: "Schedulo TM",
        descriptor: "Transport management",
        accent: [0x00, 0xC7, 0xC7],
    },
];

/// One product's entry in `releases.json`.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
pub struct Published {
    pub version: Option<String>,
    pub released: Option<String>,
    /// Path of the program in this repository, e.g. `apps/TXC/Schedulo-TXC.exe`.
    pub file: Option<String>,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
struct Releases {
    schema: u32,
    #[serde(default)]
    products: BTreeMap<String, Published>,
}

/// Read `releases.json`. Products without a version are left out.
pub fn parse(text: &str) -> Result<BTreeMap<String, Published>, String> {
    let r: Releases =
        serde_json::from_str(text).map_err(|e| format!("releases.json is not valid: {e}"))?;
    if r.schema > 1 {
        return Err("releases.json needs a newer Schedulo".into());
    }
    Ok(r.products
        .into_iter()
        .filter(|(_, p)| {
            p.version
                .as_deref()
                .is_some_and(|v| parse_version(v).is_some())
        })
        .collect())
}

/// Open `path` (e.g. `releases.json`) in Schedulo-Releases. Setting
/// `SCHEDULO_RELEASES` to a folder reads a local copy of the repository
/// instead, for trying a release before publishing it.
pub fn open(agent: &ureq::Agent, path: &str) -> Result<Box<dyn std::io::Read + Send>, String> {
    let path = path.trim_start_matches('/');
    match std::env::var("SCHEDULO_RELEASES") {
        Ok(dir) if !dir.trim().is_empty() => {
            let file = std::path::Path::new(dir.trim()).join(path);
            std::fs::File::open(&file)
                .map(|f| Box::new(f) as Box<dyn std::io::Read + Send>)
                .map_err(|e| format!("could not read {}: {e}", file.display()))
        }
        _ => agent
            .get(&format!("{RAW_BASE}{path}"))
            .call()
            .map(|r| Box::new(r.into_reader()) as Box<dyn std::io::Read + Send>)
            .map_err(|_| "could not reach Schedulo-Releases".to_string()),
    }
}

/// Download `releases.json` (blocking; a few seconds at most).
pub fn fetch(agent: &ureq::Agent) -> Result<BTreeMap<String, Published>, String> {
    let mut text = String::new();
    open(agent, "releases.json")?
        .take(1024 * 1024)
        .read_to_string(&mut text)
        .map_err(|e| e.to_string())?;
    parse(&text)
}

/// "v1.2.3" or "1.2" -> (1, 2, 3) / (1, 2, 0); pre-release suffixes ignored.
pub fn parse_version(s: &str) -> Option<(u32, u32, u32)> {
    let core = s.trim().trim_start_matches(['v', 'V']);
    let core = core.split(['-', '+']).next()?;
    let mut parts = core.split('.').map(|p| p.parse::<u32>().ok());
    let major = parts.next()??;
    let minor = parts.next().unwrap_or(Some(0))?;
    let patch = parts.next().unwrap_or(Some(0))?;
    Some((major, minor, patch))
}

/// Is `published` newer than `installed` (or is nothing installed)?
pub fn is_newer(published: &str, installed: Option<&str>) -> bool {
    match (parse_version(published), installed.and_then(parse_version)) {
        (Some(p), Some(i)) => p > i,
        (Some(_), None) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_releases_json() {
        let text = r#"{"schema":1,"updated":"2026-10-22","products":{
            "TXC":{"name":"Schedulo TXC","version":"0.5.0","file":"apps/TXC/Schedulo-TXC.exe","extra":1},
            "BUS":{"version":null,"file":null}}}"#;
        let p = parse(text).unwrap();
        assert_eq!(p.len(), 1);
        assert_eq!(p["TXC"].version.as_deref(), Some("0.5.0"));
        assert_eq!(p["TXC"].file.as_deref(), Some("apps/TXC/Schedulo-TXC.exe"));
        assert!(parse(r#"{"schema":2,"products":{}}"#).is_err());
        assert!(parse("nope").is_err());
    }

    #[test]
    fn versions() {
        assert!(is_newer("0.5.0", Some("0.4.9")));
        assert!(is_newer("0.5.0", None));
        assert!(!is_newer("0.5.0", Some("0.5.0")));
        assert!(!is_newer("0.5.0", Some("v1.0")));
        assert_eq!(parse_version("v0.3.0-rc.1"), Some((0, 3, 0)));
        assert_eq!(parse_version("x"), None);
    }

    #[test]
    fn products_are_unique() {
        for (i, a) in PRODUCTS.iter().enumerate() {
            for b in &PRODUCTS[i + 1..] {
                assert_ne!(a.short, b.short);
            }
        }
    }
}
