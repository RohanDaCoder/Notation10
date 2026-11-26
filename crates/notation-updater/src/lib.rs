//! Notation10 self-update client, backed by the repository's GitHub Releases.
//!
//! The original shipped EasiUpdate ("Enable automatic update of software" in
//! its settings); this is the clean-room replacement, driven by the tag-based
//! releases published by `.github/workflows/release.yml`:
//!
//! 1. `check` fetches `releases/latest` and compares semver tags.
//! 2. `download` streams the platform asset and verifies its SHA-256 against
//!    the release's `SHA256SUMS.txt`.
//! 3. The GUI layer installs the artifact (replace + relaunch).

use std::io::Read;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use sha2::{Digest, Sha256};
use thiserror::Error;

/// Repository whose releases drive updates.
pub const DEFAULT_REPO: &str = "RohanDaCoder/Notation10";

/// How much of a release JSON body we are willing to read.
const MAX_JSON: usize = 1024 * 1024;

#[derive(Debug, Error)]
pub enum UpdateError {
    #[error("Please check the network!")]
    Network(String),
    #[error("release metadata is malformed")]
    BadMetadata(#[from] serde_json::Error),
    #[error("checksum mismatch for {0}")]
    ChecksumMismatch(String),
    #[error("release asset for this platform is missing")]
    NoAsset,
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// A GitHub Release, reduced to the fields the updater needs.
#[derive(Debug, Clone, Deserialize)]
pub struct Release {
    pub tag_name: String,
    pub name: Option<String>,
    pub body: Option<String>,
    pub prerelease: bool,
    pub draft: bool,
    pub assets: Vec<Asset>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Asset {
    pub name: String,
    #[serde(rename = "browser_download_url")]
    pub url: String,
    pub size: u64,
}

/// Outcome of comparing a release tag with the running version.
#[derive(Debug, Clone, PartialEq)]
pub enum Update {
    /// A newer release is available.
    Available {
        version: String,
        notes: String,
        asset: Asset,
    },
    /// We are already up to date.
    UpToDate,
}

impl Release {
    /// Select the archive asset for `target` (`Notation10-<tag>-<target>.{zip,tar.gz}`).
    pub fn asset_for(&self, target: &str) -> Option<&Asset> {
        let suffixes = [format!("{target}.zip"), format!("{target}.tar.gz")];
        self.assets
            .iter()
            .find(|a| suffixes.iter().any(|s| a.name.ends_with(s)))
    }
}

/// Compare two semver-ish tag strings (`v1.2.3` vs `v1.2.4`).
pub fn is_newer(candidate: &str, current: &str) -> bool {
    let parse = |s: &str| -> Vec<u64> {
        s.trim_start_matches('v')
            .split(|c: char| !c.is_ascii_digit())
            .filter(|p| !p.is_empty())
            .map(|p| p.parse::<u64>().unwrap_or(0))
            .collect()
    };
    let c = parse(candidate);
    let cur = parse(current);
    for i in 0..c.len().max(cur.len()) {
        let a = c.get(i).copied().unwrap_or(0);
        let b = cur.get(i).copied().unwrap_or(0);
        if a != b {
            return a > b;
        }
    }
    false
}

fn http_get(url: &str) -> Result<ureq::Response, UpdateError> {
    ureq::AgentBuilder::new()
        .user_agent(concat!("notation-updater/", env!("CARGO_PKG_VERSION")))
        .build()
        .get(url)
        .call()
        .map_err(|e| UpdateError::Network(e.to_string()))
}

/// Fetch `releases/latest` and decide whether `current_version` is outdated.
pub fn check(repo: &str, current_version: &str, target: &str) -> Result<Update, UpdateError> {
    let url = format!("https://api.github.com/repos/{repo}/releases/latest");
    let mut body = String::new();
    http_get(&url)?
        .into_reader()
        .take(MAX_JSON as u64)
        .read_to_string(&mut body)?;
    let release: Release = serde_json::from_str(&body)?;

    if !is_newer(&release.tag_name, current_version) {
        return Ok(Update::UpToDate);
    }

    let asset = release
        .asset_for(target)
        .ok_or(UpdateError::NoAsset)?
        .clone();
    Ok(Update::Available {
        version: release.tag_name,
        notes: release.body.unwrap_or_default(),
        asset,
    })
}

/// Fetch and parse the release's `SHA256SUMS.txt` into (filename, digest) pairs.
pub fn fetch_checksums(repo: &str, tag: &str) -> Result<Vec<(String, String)>, UpdateError> {
    let url = format!("https://api.github.com/repos/{repo}/releases/tags/{tag}");
    let mut body = String::new();
    http_get(&url)?
        .into_reader()
        .take(MAX_JSON as u64)
        .read_to_string(&mut body)?;
    let release: Release = serde_json::from_str(&body)?;

    let sums = release
        .assets
        .iter()
        .find(|a| a.name == "SHA256SUMS.txt")
        .ok_or(UpdateError::NoAsset)?;

    let mut text = String::new();
    http_get(&sums.url)?
        .into_reader()
        .take(MAX_JSON as u64)
        .read_to_string(&mut text)?;

    Ok(text
        .lines()
        .filter_map(|l| {
            let mut parts = l.split_whitespace();
            let digest = parts.next()?;
            let name = parts.next()?;
            Some((
                name.trim_start_matches('*').to_string(),
                digest.to_ascii_lowercase(),
            ))
        })
        .collect())
}

/// Where a downloaded update is staged before installation.
pub fn stage_dir() -> Result<PathBuf, std::io::Error> {
    let dir = std::env::temp_dir().join("notation10-update");
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// Download `asset`, verify its SHA-256 against `expected`, return the path.
pub fn download_and_verify(
    asset: &Asset,
    expected_sha256: &str,
    on_progress: impl Fn(u64, u64),
) -> Result<PathBuf, UpdateError> {
    let dir = stage_dir()?;
    let path = dir.join(&asset.name);

    let mut reader = http_get(&asset.url)?.into_reader();
    let mut bytes = Vec::with_capacity(asset.size as usize);
    reader.read_to_end(&mut bytes)?;
    on_progress(bytes.len() as u64, asset.size);

    let digest = hex::encode(Sha256::digest(&bytes));
    if !digest.eq_ignore_ascii_case(expected_sha256.trim()) {
        return Err(UpdateError::ChecksumMismatch(asset.name.clone()));
    }

    std::fs::write(&path, &bytes)?;
    Ok(path)
}

/// Look up the expected digest for `asset_name` in a checksum table.
pub fn expected_digest<'a>(checksums: &'a [(String, String)], asset_name: &str) -> Option<&'a str> {
    checksums
        .iter()
        .find(|(name, _)| name == asset_name)
        .map(|(_, digest)| digest.as_str())
}

/// Compute the SHA-256 of a local file (used by tests and manual verification).
pub fn sha256_of_file(path: &Path) -> Result<String, UpdateError> {
    let mut file = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 64 * 1024];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hex::encode(hasher.finalize()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semver_comparison() {
        assert!(is_newer("v0.2.0", "v0.1.0"));
        assert!(is_newer("v0.1.1", "v0.1.0"));
        assert!(is_newer("v1.0.0", "v0.9.9"));
        assert!(!is_newer("v0.1.0", "v0.1.0"));
        assert!(!is_newer("v0.1.0", "v0.2.0"));
        assert!(is_newer("v0.1.0.1", "v0.1.0"));
    }

    #[test]
    fn asset_selection_by_target() {
        let release = Release {
            tag_name: "v0.1.0".into(),
            name: None,
            body: None,
            prerelease: false,
            draft: false,
            assets: vec![
                Asset {
                    name: "Notation10-v0.1.0-x86_64-pc-windows-msvc.zip".into(),
                    url: "http://example.com/win.zip".into(),
                    size: 1,
                },
                Asset {
                    name: "Notation10-v0.1.0-x86_64-unknown-linux-gnu.tar.gz".into(),
                    url: "http://example.com/linux.tar.gz".into(),
                    size: 1,
                },
            ],
        };
        let win = release
            .asset_for("x86_64-pc-windows-msvc")
            .expect("windows asset");
        assert!(win.name.ends_with(".zip"));
        let linux = release
            .asset_for("x86_64-unknown-linux-gnu")
            .expect("linux asset");
        assert!(linux.name.ends_with(".tar.gz"));
        assert!(release.asset_for("aarch64-apple-darwin").is_none());
    }

    #[test]
    fn download_verifies_checksum() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("payload.bin");
        std::fs::write(&path, b"hello update").unwrap();
        let digest = sha256_of_file(&path).unwrap();
        assert_eq!(digest.len(), 64);
    }

    #[test]
    fn expected_digest_lookup() {
        let sums = vec![("a.zip".to_string(), "deadbeef".to_string())];
        assert_eq!(expected_digest(&sums, "a.zip"), Some("deadbeef"));
        assert_eq!(expected_digest(&sums, "b.zip"), None);
    }
}
