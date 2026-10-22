//! Build script:
//!
//! * Injects build information through `cargo:rustc-env`: `APP_COPYRIGHT`
//!   (the copyright line), `APP_GIT_HASH` (short commit hash) and
//!   `APP_BUILD_DATE` (UTC, `YYYY-MM-DD`). Both build values fall back to
//!   "unknown", e.g. when building from a source zip without git.
//! * On Windows, embeds the application icon (`assets/app.ico`, if present)
//!   and version information into the executable. Failure to compile
//!   resources never fails the build; a cargo warning is printed instead.

use std::path::{Path, PathBuf};
use std::process::Command;

/// Year shown in the copyright line. Update at release time; this is the only
/// place it is set.
const COPYRIGHT_YEAR: u32 = 2026;
const COMPANY: &str = "Newey Transport Services";
/// Product name shown in the Windows file properties.
const PRODUCT_NAME: &str = "Schedulo";

fn main() {
    println!("cargo:rerun-if-changed=assets/app.ico");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH");

    let copyright = format!("© {COMPANY} {COPYRIGHT_YEAR}. All rights reserved.");
    println!("cargo:rustc-env=APP_COPYRIGHT={copyright}");
    println!("cargo:rustc-env=APP_GIT_HASH={}", git_hash());
    println!("cargo:rustc-env=APP_BUILD_DATE={}", build_date());

    #[cfg(windows)]
    windows_resources(&copyright);
}

/// Runs git in the crate directory; `None` when git is missing or fails.
fn git(args: &[&str]) -> Option<String> {
    let dir = std::env::var_os("CARGO_MANIFEST_DIR")?;
    let out = Command::new("git")
        .args(args)
        .current_dir(&dir)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8(out.stdout).ok()?.trim().to_string();
    (!s.is_empty()).then_some(s)
}

/// Short commit hash of this checkout, or "unknown". Only used when the crate
/// directory is itself the top of the git work tree, so that a source zip
/// unpacked inside some other repository does not report that repository's
/// commit.
fn git_hash() -> String {
    let manifest = std::env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from);
    let top = git(&["rev-parse", "--show-toplevel"]).map(PathBuf::from);
    let same = match (manifest, top) {
        (Some(m), Some(t)) => match (m.canonicalize(), t.canonicalize()) {
            (Ok(a), Ok(b)) => a == b,
            _ => false,
        },
        _ => false,
    };
    if !same {
        return "unknown".into();
    }
    // Re-run when HEAD moves (new commit or checkout). Only existing files are
    // listed: a missing path would make cargo re-run the script every build.
    for p in [
        git(&["rev-parse", "--git-path", "HEAD"]),
        git(&["symbolic-ref", "-q", "HEAD"])
            .and_then(|r| git(&["rev-parse", "--git-path", r.as_str()])),
        git(&["rev-parse", "--git-path", "packed-refs"]),
    ]
    .into_iter()
    .flatten()
    {
        let dir = std::env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from);
        let path = match dir {
            Some(d) if Path::new(&p).is_relative() => d.join(&p),
            _ => PathBuf::from(&p),
        };
        if path.exists() {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
    git(&["rev-parse", "--short=8", "HEAD"]).unwrap_or_else(|| "unknown".into())
}

/// Build date (UTC) as `YYYY-MM-DD`; honours `SOURCE_DATE_EPOCH` for
/// reproducible builds.
fn build_date() -> String {
    let secs = std::env::var("SOURCE_DATE_EPOCH")
        .ok()
        .and_then(|s| s.trim().parse::<i64>().ok())
        .or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .ok()
                .and_then(|d| i64::try_from(d.as_secs()).ok())
        });
    match secs {
        Some(s) => {
            let (y, m, d) = civil_from_days(s.div_euclid(86_400));
            format!("{y:04}-{m:02}-{d:02}")
        }
        None => "unknown".into(),
    }
}

/// Days since 1970-01-01 to a proleptic Gregorian (year, month, day).
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = yoe + era * 400 + i64::from(m <= 2);
    (y, m, d)
}

#[cfg(windows)]
fn windows_resources(copyright: &str) {
    let version = std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.1.0".into());
    let mut parts: Vec<u64> = version
        .split(['.', '-'])
        .filter_map(|p| p.parse().ok())
        .take(3)
        .collect();
    parts.resize(4, 0);
    let packed = (parts[0] << 48) | (parts[1] << 32) | (parts[2] << 16) | parts[3];

    let mut res = winresource::WindowsResource::new();
    if Path::new("assets/app.ico").exists() {
        res.set_icon("assets/app.ico");
    }
    res.set("ProductName", PRODUCT_NAME)
        .set("FileDescription", PRODUCT_NAME)
        .set("CompanyName", COMPANY)
        .set("LegalCopyright", copyright)
        .set("OriginalFilename", "Schedulo.exe")
        .set("InternalName", "Schedulo")
        .set("ProductVersion", &version)
        .set("FileVersion", &version)
        .set_version_info(winresource::VersionInfo::PRODUCTVERSION, packed)
        .set_version_info(winresource::VersionInfo::FILEVERSION, packed);
    if let Err(e) = res.compile() {
        println!("cargo:warning=could not embed Windows resources (icon/version info): {e}");
    }
}
