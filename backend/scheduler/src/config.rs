use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use yams_scheduler::YamsSchedulerConfig;

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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SchedulerDaemonConfig {
    pub database_url: String,
    pub object_store_dir: PathBuf,
    pub scheduler: YamsSchedulerConfig,
}

impl Default for SchedulerDaemonConfig {
    fn default() -> Self {
        Self {
            database_url: "yams.db".into(),
            object_store_dir: PathBuf::from("storage/"),
            scheduler: YamsSchedulerConfig {
                enabled: true,
                ..YamsSchedulerConfig::default()
            },
        }
    }
}

pub fn load(config_path: Option<PathBuf>) -> Result<SchedulerDaemonConfig, ConfigError> {
    let explicit = config_path.is_some();
    let path = config_path.unwrap_or_else(|| PathBuf::from(DEFAULT_CONFIG_FILE));
    load_file(&path, explicit)
}

fn load_file(path: &Path, explicit: bool) -> Result<SchedulerDaemonConfig, ConfigError> {
    match std::fs::read_to_string(path) {
        Ok(contents) => match parse_config_file::<SchedulerDaemonConfig>(path, &contents) {
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
                    Ok(SchedulerDaemonConfig::default())
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
                Ok(SchedulerDaemonConfig::default())
            }
        }
        Err(source) => Err(ConfigError::Io {
            path: path.display().to_string(),
            source,
        }),
    }
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
