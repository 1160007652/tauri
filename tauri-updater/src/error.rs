use thiserror::Error as DeriveError;

use {anyhow, base64, minisign_verify, reqwest, semver, serde_json};

#[derive(Debug, DeriveError)]
pub enum Error {
  // Error catcher
  #[error("{0}")]
  Bundler(#[from] anyhow::Error),
  #[error("{0}")]
  Reqwest(#[from] reqwest::Error),
  #[error("{0}")]
  Semver(#[from] semver::SemVerError),
  #[error("{0}")]
  SerdeJson(#[from] serde_json::Error),
  #[error("{0}")]
  Minisign(#[from] minisign_verify::Error),
  #[error("{0}")]
  Base64(#[from] base64::DecodeError),
  #[error("{0}")]
  Utf8(#[from] std::str::Utf8Error),

  // Custom
  #[error("{0}")]
  Release(String),
  #[error("{0}")]
  Config(String),
  #[error("{0}")]
  Network(String),
  #[error("{0}")]
  Updater(String),
  #[error("No updates available: {0}")]
  UpToDate(String),
}

pub type Result<T = ()> = anyhow::Result<T, Error>;
