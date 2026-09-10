use std::path::{Path, PathBuf};

use chrono_tz::Tz;
use clap::Parser;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const DEFAULT_CONFIG_FILE: &str = "yams-scheduler.json";

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("config file `{path}` does not exist")]
    NotFound { path: String },
    #[error("failed to read config file `{path}`: {source}")]
    Io {
        path: String,
        source: std::io::Error,
    },
    #[error("config file `{path}` is malformed: {message}")]
    Malformed { path: String, message: String },
    #[error("invalid scheduler timezone `{timezone}`")]
    InvalidTimezone { timezone: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedConfig {
    pub database_url: String,
    pub object_store_dir: PathBuf,
    pub log_dir: Option<PathBuf>,
    pub timezone: Tz,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
struct DaemonConfig {
    database_url: String,
    object_store_dir: PathBuf,
    log_dir: Option<PathBuf>,
    timezone: Option<String>,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            database_url: "yams.db".into(),
            object_store_dir: PathBuf::from("storage/"),
            log_dir: None,
            timezone: None,
        }
    }
}

#[derive(Debug, Parser, Default, PartialEq, Eq)]
pub struct CliOverlay {
    pub config_path: Option<PathBuf>,
    pub database_url: Option<String>,
    pub object_store_dir: Option<PathBuf>,
    pub log_dir: Option<PathBuf>,
    pub timezone: Option<String>,
}

pub fn resolve(cli: CliOverlay) -> Result<ResolvedConfig, ConfigError> {
    let explicit = cli.config_path.is_some();
    let path = cli
        .config_path
        .clone()
        .unwrap_or_else(|| PathBuf::from(DEFAULT_CONFIG_FILE));
    let file = load_file(&path, explicit)?;
    overlay(file, &cli)
}

fn load_file(path: &Path, explicit: bool) -> Result<DaemonConfig, ConfigError> {
    match std::fs::read_to_string(path) {
        Ok(contents) => match parse_config_file::<DaemonConfig>(path, &contents) {
            Ok(config) => Ok(config),
            Err(message) => {
                if explicit {
                    Err(ConfigError::Malformed {
                        path: path.display().to_string(),
                        message,
                    })
                } else {
                    tracing::warn!(
                        path = %path.display(),
                        error = %message,
                        "config file is malformed; using defaults"
                    );
                    Ok(DaemonConfig::default())
                }
            }
        },
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            if explicit {
                Err(ConfigError::NotFound {
                    path: path.display().to_string(),
                })
            } else {
                tracing::warn!(
                    path = %path.display(),
                    "config file does not exist; using defaults"
                );
                Ok(DaemonConfig::default())
            }
        }
        Err(source) => Err(ConfigError::Io {
            path: path.display().to_string(),
            source,
        }),
    }
}

fn overlay(mut file: DaemonConfig, cli: &CliOverlay) -> Result<ResolvedConfig, ConfigError> {
    if let Some(database_url) = &cli.database_url {
        file.database_url = database_url.clone();
    }
    if let Some(object_store_dir) = &cli.object_store_dir {
        file.object_store_dir = object_store_dir.clone();
    }
    if let Some(log_dir) = &cli.log_dir {
        file.log_dir = Some(log_dir.clone());
    }

    let timezone_raw = cli
        .timezone
        .clone()
        .or(file.timezone)
        .unwrap_or_else(|| "Europe/Vienna".into());
    let timezone = timezone_raw
        .parse::<Tz>()
        .map_err(|_| ConfigError::InvalidTimezone {
            timezone: timezone_raw,
        })?;

    Ok(ResolvedConfig {
        database_url: file.database_url,
        object_store_dir: file.object_store_dir,
        log_dir: file.log_dir,
        timezone,
    })
}

fn parse_config_file<T: serde::de::DeserializeOwned>(
    path: &Path,
    contents: &str,
) -> Result<T, String> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("toml") => toml::from_str(contents).map_err(|err| err.to_string()),
        Some("json") => serde_json::from_str(contents).map_err(|err| err.to_string()),
        Some(ext) => Err(format!(
            "unsupported config format `.{ext}`; use `.json` or `.toml`"
        )),
        None => serde_json::from_str(contents).map_err(|err| err.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn overlays_database_url_from_cli() {
        let resolved = overlay(
            DaemonConfig::default(),
            &CliOverlay {
                database_url: Some("env.db".into()),
                ..CliOverlay::default()
            },
        )
        .unwrap();
        assert_eq!(resolved.database_url, "env.db");
    }

    #[test_log::test]
    fn parses_timezone_from_file() {
        let dir = std::env::temp_dir().join(format!(
            "yams-scheduler-config-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("config.json");
        std::fs::write(
            &path,
            r#"{
                "databaseUrl": "file.db",
                "timezone": "Europe/Berlin"
            }"#,
        )
        .unwrap();

        let resolved = resolve(CliOverlay {
            config_path: Some(path),
            ..CliOverlay::default()
        })
        .unwrap();
        assert_eq!(resolved.database_url, "file.db");
        assert_eq!(resolved.timezone, chrono_tz::Europe::Berlin);

        std::fs::remove_dir_all(&dir).ok();
    }
}
