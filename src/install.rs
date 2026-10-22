//! Folders on this computer, which version of each app is installed, and
//! downloading a newer one.
//!
//! ```text
//! %LOCALAPPDATA%\Schedulo\
//!   apps\TXC\Schedulo-TXC.exe     one folder per product
//!   downloads\                    partial downloads (cleared on success)
//!   installed.txt                 TXC=0.5.0, one line per product
//! %APPDATA%\Schedulo\revoked.txt  signed revoked-keys list shared by every app
//! ```
//!
//! Everything is created on first start.

use std::collections::BTreeMap;
use std::io::Read;
use std::path::{Path, PathBuf};

fn env_dir(vars: &[&str]) -> Option<PathBuf> {
    vars.iter()
        .filter_map(|v| std::env::var(v).ok())
        .find(|v| !v.trim().is_empty())
        .map(PathBuf::from)
}

/// `%LOCALAPPDATA%\Schedulo` (programs and downloads).
pub fn local_root() -> Option<PathBuf> {
    env_dir(&["LOCALAPPDATA", "XDG_DATA_HOME"])
        .or_else(|| env_dir(&["HOME"]).map(|h| h.join(".local").join("share")))
        .map(|d| d.join("Schedulo"))
}

/// `%APPDATA%\Schedulo` (shared settings such as the revoked-keys list).
pub fn shared_root() -> Option<PathBuf> {
    env_dir(&["APPDATA", "XDG_CONFIG_HOME"])
        .or_else(|| env_dir(&["HOME"]).map(|h| h.join(".config")))
        .map(|d| d.join("Schedulo"))
}

/// The folders Schedulo uses, created if missing.
#[derive(Clone, Debug)]
pub struct Folders {
    pub apps: PathBuf,
    pub downloads: PathBuf,
    pub installed_file: PathBuf,
    pub revoked_file: PathBuf,
}

impl Folders {
    pub fn create(local: &Path, shared: &Path) -> Result<Folders, String> {
        let f = Folders {
            apps: local.join("apps"),
            downloads: local.join("downloads"),
            installed_file: local.join("installed.txt"),
            revoked_file: shared.join("revoked.txt"),
        };
        for d in [&f.apps, &f.downloads, &shared.to_path_buf()] {
            std::fs::create_dir_all(d)
                .map_err(|e| format!("could not create {}: {e}", d.display()))?;
        }
        Ok(f)
    }

    pub fn app_dir(&self, short: &str) -> PathBuf {
        self.apps.join(short)
    }

    /// The installed program for `short`: the first `.exe` in its folder.
    pub fn program(&self, short: &str) -> Option<PathBuf> {
        let mut exes: Vec<PathBuf> = std::fs::read_dir(self.app_dir(short))
            .ok()?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension().is_some_and(|x| x.eq_ignore_ascii_case("exe")))
            .collect();
        exes.sort();
        exes.into_iter().next()
    }

    /// Installed versions, by product.
    pub fn installed(&self) -> BTreeMap<String, String> {
        std::fs::read_to_string(&self.installed_file)
            .unwrap_or_default()
            .lines()
            .filter_map(|l| {
                let (k, v) = l.split_once('=')?;
                Some((k.trim().to_string(), v.trim().to_string()))
            })
            .filter(|(k, v)| !k.is_empty() && !v.is_empty())
            .collect()
    }

    fn set_installed(&self, short: &str, version: &str) -> Result<(), String> {
        let mut all = self.installed();
        all.insert(short.to_string(), version.to_string());
        let text: String = all.iter().map(|(k, v)| format!("{k}={v}\n")).collect();
        std::fs::write(&self.installed_file, text)
            .map_err(|e| format!("could not save installed.txt: {e}"))
    }

    /// Install the program read from `download` as `file_name` for `short`,
    /// recording `version`. The old program is only replaced once the new
    /// one has downloaded completely; if the program is running, it is left
    /// alone and the update is tried again next time.
    pub fn install(
        &self,
        download: Box<dyn Read + Send>,
        short: &str,
        file_name: &str,
        version: &str,
    ) -> Result<(), String> {
        if file_name.contains(['/', '\\']) || file_name.starts_with('.') {
            return Err("the release has an unsafe file name".into());
        }
        let part = self.downloads.join(format!("{short}-{file_name}.part"));
        {
            let mut out = std::fs::File::create(&part)
                .map_err(|e| format!("could not save the download: {e}"))?;
            // Programs are a few tens of MB; refuse anything absurd.
            let mut reader = download.take(500 * 1024 * 1024);
            std::io::copy(&mut reader, &mut out)
                .map_err(|e| format!("the download stopped: {e}"))?;
        }
        let dir = self.app_dir(short);
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let target = dir.join(file_name);
        if target.exists() {
            // Move the old one aside first: Windows will not overwrite a
            // running program, and this keeps it if the swap fails.
            let old = dir.join(format!("{file_name}.old"));
            let _ = std::fs::remove_file(&old);
            if std::fs::rename(&target, &old).is_err() {
                let _ = std::fs::remove_file(&part);
                return Err("close it to finish updating".into());
            }
        }
        std::fs::rename(&part, &target).map_err(|e| format!("could not install: {e}"))?;
        let _ = std::fs::remove_file(dir.join(format!("{file_name}.old")));
        self.set_installed(short, version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp() -> PathBuf {
        let n = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("schedulo-test-{n}"))
    }

    #[test]
    fn creates_folders_and_remembers_versions() {
        let root = temp();
        let f = Folders::create(&root.join("local"), &root.join("shared")).unwrap();
        assert!(f.apps.is_dir() && f.downloads.is_dir());
        assert!(f.installed().is_empty());
        f.set_installed("TXC", "0.5.0").unwrap();
        f.set_installed("BUS", "1.0.0").unwrap();
        f.set_installed("TXC", "0.5.1").unwrap();
        let i = f.installed();
        assert_eq!(i["TXC"], "0.5.1");
        assert_eq!(i["BUS"], "1.0.0");
        assert_eq!(f.program("TXC"), None);
        std::fs::create_dir_all(f.app_dir("TXC")).unwrap();
        std::fs::write(f.app_dir("TXC").join("Schedulo-TXC.exe"), b"x").unwrap();
        assert!(f.program("TXC").unwrap().ends_with("Schedulo-TXC.exe"));
        let _ = std::fs::remove_dir_all(root);
    }
}
