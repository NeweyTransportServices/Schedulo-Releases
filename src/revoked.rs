//! The signed revoked-keys list (`licence/revoked.txt` here, see
//! `docs/FORMAT.md`). Schedulo downloads it, checks the owner's signature
//! and, if it is newer than the copy on this computer, saves it to
//! `%APPDATA%\Schedulo\revoked.txt` for every Schedulo app to use.
//!
//! This only checks lists; it cannot make them (that needs the owner's
//! private root key, which lives only in the Licence Manager).

use base64::Engine;
use ring::signature::{UnparsedPublicKey, ED25519};
use std::path::Path;

const HEADER: &str = "NTS-REVOCATIONS-V1";
const DOMAIN: &[u8] = b"NTS-LICENCE-V2|REVOCATIONS|";
const ROOT_PUBLIC_KEY_FILE: &str = include_str!("../licence/root_public_key.txt");

/// A checked list.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct List {
    pub issued: i64,
    pub ids: Vec<[u8; 8]>,
}

/// The owner's root public key built into Schedulo.
pub fn root_public_key() -> Option<[u8; 32]> {
    let line = ROOT_PUBLIC_KEY_FILE
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty() && !l.starts_with('#'))?;
    base64::engine::general_purpose::STANDARD
        .decode(line)
        .ok()?
        .try_into()
        .ok()
}

fn unhex(s: &str) -> Option<[u8; 8]> {
    if s.len() != 16 || !s.is_ascii() {
        return None;
    }
    let mut out = [0u8; 8];
    for (i, b) in out.iter_mut().enumerate() {
        *b = u8::from_str_radix(&s[2 * i..2 * i + 2], 16).ok()?;
    }
    Some(out)
}

/// Read a list and check the owner's signature.
pub fn verify(text: &str, root_public: &[u8; 32]) -> Option<List> {
    let mut lines = text
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'));
    if lines.next()? != HEADER {
        return None;
    }
    let issued: i64 = lines.next()?.strip_prefix("issued:")?.trim().parse().ok()?;
    let mut ids = Vec::new();
    let mut sig = None;
    for l in lines {
        if let Some(s) = l.strip_prefix("signature:") {
            sig = Some(s.trim().to_string());
            break;
        }
        ids.push(unhex(l)?);
    }
    let sig = base64::engine::general_purpose::STANDARD
        .decode(sig?)
        .ok()?;
    let mut msg = DOMAIN.to_vec();
    msg.extend_from_slice(&issued.to_be_bytes());
    for id in &ids {
        msg.extend_from_slice(id);
    }
    UnparsedPublicKey::new(&ED25519, root_public)
        .verify(&msg, &sig)
        .ok()?;
    Some(List { issued, ids })
}

/// Save `text` (already downloaded) to `path` if it is genuine and newer
/// than what is there. Returns the number of revoked keys now known, or
/// `None` if nothing changed.
pub fn store_if_newer(text: &str, path: &Path, root_public: &[u8; 32]) -> Option<usize> {
    let new = verify(text, root_public)?;
    let current = std::fs::read_to_string(path)
        .ok()
        .and_then(|t| verify(&t, root_public));
    if current.as_ref().is_some_and(|c| c.issued >= new.issued) {
        return None;
    }
    let tmp = path.with_extension("tmp");
    std::fs::write(&tmp, text).ok()?;
    std::fs::rename(&tmp, path).ok()?;
    Some(new.ids.len())
}

/// Download the published list and store it if newer.
pub fn update(agent: &ureq::Agent, path: &Path) -> Option<usize> {
    let key = root_public_key()?;
    let mut text = String::new();
    std::io::Read::read_to_string(
        &mut std::io::Read::take(
            crate::catalogue::open(agent, "licence/revoked.txt").ok()?,
            1024 * 1024,
        ),
        &mut text,
    )
    .ok()?;
    store_if_newer(&text, path, &key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ring::signature::{Ed25519KeyPair, KeyPair};

    fn signed(pair: &Ed25519KeyPair, issued: i64, ids: &[[u8; 8]]) -> String {
        let mut msg = DOMAIN.to_vec();
        msg.extend_from_slice(&issued.to_be_bytes());
        let mut out = format!("{HEADER}\nissued: {issued}\n");
        for id in ids {
            msg.extend_from_slice(id);
            out.push_str(&id.iter().map(|b| format!("{b:02x}")).collect::<String>());
            out.push('\n');
        }
        let sig = pair.sign(&msg);
        out.push_str("signature: ");
        out.push_str(&base64::engine::general_purpose::STANDARD.encode(sig.as_ref()));
        out
    }

    #[test]
    fn built_in_key_is_readable() {
        assert!(root_public_key().is_some());
    }

    #[test]
    fn only_genuine_newer_lists_are_stored() {
        let pair = Ed25519KeyPair::from_seed_unchecked(&[7; 32]).unwrap();
        let public: [u8; 32] = pair.public_key().as_ref().try_into().unwrap();
        let dir = std::env::temp_dir().join(format!("schedulo-rev-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("revoked.txt");

        let v1 = signed(&pair, 100, &[[1; 8]]);
        assert_eq!(store_if_newer(&v1, &path, &public), Some(1));
        // Same or older: ignored.
        assert_eq!(store_if_newer(&v1, &path, &public), None);
        assert_eq!(
            store_if_newer(&signed(&pair, 50, &[]), &path, &public),
            None
        );
        // Newer: stored.
        let v2 = signed(&pair, 200, &[[1; 8], [2; 8]]);
        assert_eq!(store_if_newer(&v2, &path, &public), Some(2));
        // Tampered: refused.
        let tampered = v2.replace("0202020202020202\n", "");
        assert_eq!(verify(&tampered, &public), None);
        // Signed by someone else: refused.
        let other = Ed25519KeyPair::from_seed_unchecked(&[8; 32]).unwrap();
        assert_eq!(
            store_if_newer(&signed(&other, 999, &[]), &path, &public),
            None
        );
        let _ = std::fs::remove_dir_all(dir);
    }
}
