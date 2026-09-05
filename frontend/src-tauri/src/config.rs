use std::path::{Path, PathBuf};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

const DEFAULT_CONFIG_FILE_NAME: &str = "yams.json";

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("config file `{path}` does not exist")]
    NotFound { path: String },
    #[error("failed to read config file `{path}`: {source}")]
    Io {
        path: String,
        source: std::io::Error,
    },
    #[error("config file `{path}` is malformed: {source}")]
    Malformed {
        path: String,
        source: serde_json::Error,
    },
    #[error("YAMS_MODE=embedded requires YAMS_DATABASE_URL and YAMS_OBJECT_STORE_DIR")]
    EmbeddedEnvIncomplete,
    #[error("YAMS_MODE=remote requires YAMS_REMOTE_API_URL")]
    RemoteEnvIncomplete,
    #[error("YAMS_DATABASE_URL and YAMS_OBJECT_STORE_DIR are only valid when mode is embedded")]
    EmbeddedEnvInRemoteMode,
    #[error("YAMS_REMOTE_API_URL is only valid when mode is remote")]
    RemoteEnvInEmbeddedMode,
    #[error("invalid YAMS_MODE `{0}`; expected `embedded` or `remote`")]
    InvalidMode(String),
    #[error("invalid YAMS_REMOTE_API_URL: {0}")]
    InvalidRemoteApiUrl(String),
}

pub fn project_dirs() -> ProjectDirs {
    ProjectDirs::from("at", "HTL Grieskirchen", "YAMS").expect("unsupported OS")
}

pub fn default_log_dir() -> PathBuf {
    project_dirs().data_dir().join("logs")
}

/// Resolve log directory before tracing init (env beats config file beats default).
pub fn resolve_log_dir() -> PathBuf {
    if let Some(dir) = std::env::var_os("YAMS_LOG_DIR") {
        return PathBuf::from(dir);
    }
    let path = std::env::var_os("YAMS_CONFIG_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(default_config_path);
    peek_log_dir_from_file(&path).unwrap_or_else(default_log_dir)
}

fn peek_log_dir_from_file(path: &Path) -> Option<PathBuf> {
    let contents = std::fs::read_to_string(path).ok()?;
    let value: serde_json::Value = serde_json::from_str(&contents).ok()?;
    value
        .get("logDir")
        .and_then(|v| v.as_str())
        .map(PathBuf::from)
}

fn resolve_log_dir_overlay(file: Option<PathBuf>, env: Option<PathBuf>) -> PathBuf {
    env.or(file).unwrap_or_else(default_log_dir)
}

fn default_config_path() -> PathBuf {
    project_dirs().config_dir().join(DEFAULT_CONFIG_FILE_NAME)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeploymentMode {
    Embedded {
        database_url: String,
        object_store_dir: PathBuf,
    },
    Remote {
        remote_api_url: Url,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TauriConfig {
    pub deployment: DeploymentMode,
    pub dev: bool,
    pub log_dir: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "camelCase")]
enum TauriFileConfig {
    #[serde(rename_all = "camelCase")]
    Embedded {
        database_url: String,
        object_store_dir: PathBuf,
        #[serde(default)]
        dev: bool,
        #[serde(default)]
        log_dir: Option<PathBuf>,
    },
    #[serde(rename_all = "camelCase")]
    Remote {
        remote_api_url: Url,
        #[serde(default)]
        dev: bool,
        #[serde(default)]
        log_dir: Option<PathBuf>,
    },
}

impl From<TauriFileConfig> for TauriConfig {
    fn from(file: TauriFileConfig) -> Self {
        match file {
            TauriFileConfig::Embedded {
                database_url,
                object_store_dir,
                dev,
                log_dir,
            } => Self {
                deployment: DeploymentMode::Embedded {
                    database_url,
                    object_store_dir,
                },
                dev,
                log_dir: log_dir.unwrap_or_else(default_log_dir),
            },
            TauriFileConfig::Remote {
                remote_api_url,
                dev,
                log_dir,
            } => Self {
                deployment: DeploymentMode::Remote { remote_api_url },
                dev,
                log_dir: log_dir.unwrap_or_else(default_log_dir),
            },
        }
    }
}

impl TauriConfig {
    pub fn production_default() -> Self {
        let data = project_dirs().data_dir().to_path_buf();
        Self {
            deployment: DeploymentMode::Embedded {
                database_url: data.join("yams.db").to_string_lossy().into_owned(),
                object_store_dir: data.join("objects"),
            },
            dev: false,
            log_dir: default_log_dir(),
        }
    }

    pub fn frontend_dto(&self) -> FrontendConfigDto {
        match &self.deployment {
            DeploymentMode::Embedded { .. } => FrontendConfigDto::Embedded { dev: self.dev },
            DeploymentMode::Remote { remote_api_url } => FrontendConfigDto::Remote {
                remote_api_url: remote_api_url.to_string(),
                dev: self.dev,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "mode", rename_all = "camelCase")]
pub enum FrontendConfigDto {
    Embedded {
        dev: bool,
    },
    #[serde(rename_all = "camelCase")]
    Remote {
        remote_api_url: String,
        dev: bool,
    },
}

#[derive(Debug, Default)]
struct EnvOverlay {
    mode: Option<ModeKind>,
    database_url: Option<String>,
    object_store_dir: Option<PathBuf>,
    remote_api_url: Option<String>,
    dev: Option<bool>,
    log_dir: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModeKind {
    Embedded,
    Remote,
}

pub fn load() -> Result<TauriConfig, ConfigError> {
    let path = std::env::var_os("YAMS_CONFIG_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(default_config_path);
    let explicit = std::env::var_os("YAMS_CONFIG_PATH").is_some();
    let file = load_file(&path, explicit)?;
    overlay(file, env_overlay()?)
}

fn env_overlay() -> Result<EnvOverlay, ConfigError> {
    Ok(EnvOverlay {
        mode: match std::env::var("YAMS_MODE") {
            Ok(value) => Some(parse_mode(&value)?),
            Err(_) => None,
        },
        database_url: std::env::var("YAMS_DATABASE_URL").ok(),
        object_store_dir: std::env::var_os("YAMS_OBJECT_STORE_DIR").map(PathBuf::from),
        remote_api_url: std::env::var("YAMS_REMOTE_API_URL").ok(),
        dev: std::env::var("YAMS_DEV")
            .ok()
            .map(|value| parse_bool(&value)),
        log_dir: std::env::var_os("YAMS_LOG_DIR").map(PathBuf::from),
    })
}

fn parse_mode(value: &str) -> Result<ModeKind, ConfigError> {
    match value {
        "embedded" => Ok(ModeKind::Embedded),
        "remote" => Ok(ModeKind::Remote),
        other => Err(ConfigError::InvalidMode(other.to_string())),
    }
}

fn parse_bool(value: &str) -> bool {
    matches!(value.to_ascii_lowercase().as_str(), "1" | "true" | "yes")
}

fn load_file(path: &Path, explicit: bool) -> Result<TauriConfig, ConfigError> {
    match std::fs::read_to_string(path) {
        Ok(contents) => match serde_json::from_str::<TauriFileConfig>(&contents) {
            Ok(file) => Ok(TauriConfig::from(file)),
            Err(source) => {
                if explicit {
                    Err(ConfigError::Malformed {
                        path: path.display().to_string(),
                        source,
                    })
                } else {
                    tracing::warn!(
                        path = %path.display(),
                        error = %source,
                        "config file is malformed; using defaults"
                    );
                    Ok(TauriConfig::production_default())
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
                Ok(TauriConfig::production_default())
            }
        }
        Err(source) => Err(ConfigError::Io {
            path: path.display().to_string(),
            source,
        }),
    }
}

fn overlay(config: TauriConfig, env: EnvOverlay) -> Result<TauriConfig, ConfigError> {
    let current = match &config.deployment {
        DeploymentMode::Embedded { .. } => ModeKind::Embedded,
        DeploymentMode::Remote { .. } => ModeKind::Remote,
    };
    let target = env.mode.unwrap_or(current);

    let has_embedded_env = env.database_url.is_some() || env.object_store_dir.is_some();
    let has_remote_env = env.remote_api_url.is_some();

    match target {
        ModeKind::Embedded => {
            if has_remote_env {
                return Err(ConfigError::RemoteEnvInEmbeddedMode);
            }
            let (database_url, object_store_dir) = match config.deployment {
                DeploymentMode::Embedded {
                    database_url,
                    object_store_dir,
                } => (
                    env.database_url.unwrap_or(database_url),
                    env.object_store_dir.unwrap_or(object_store_dir),
                ),
                DeploymentMode::Remote { .. } => {
                    let database_url =
                        env.database_url.ok_or(ConfigError::EmbeddedEnvIncomplete)?;
                    let object_store_dir = env
                        .object_store_dir
                        .ok_or(ConfigError::EmbeddedEnvIncomplete)?;
                    (database_url, object_store_dir)
                }
            };
            Ok(TauriConfig {
                deployment: DeploymentMode::Embedded {
                    database_url,
                    object_store_dir,
                },
                dev: env.dev.unwrap_or(config.dev),
                log_dir: resolve_log_dir_overlay(
                    Some(config.log_dir),
                    env.log_dir,
                ),
            })
        }
        ModeKind::Remote => {
            if has_embedded_env {
                return Err(ConfigError::EmbeddedEnvInRemoteMode);
            }
            let remote_api_url = match config.deployment {
                DeploymentMode::Remote { remote_api_url } => match env.remote_api_url {
                    Some(value) => parse_url(&value)?,
                    None => remote_api_url,
                },
                DeploymentMode::Embedded { .. } => {
                    let value = env.remote_api_url.ok_or(ConfigError::RemoteEnvIncomplete)?;
                    parse_url(&value)?
                }
            };
            Ok(TauriConfig {
                deployment: DeploymentMode::Remote { remote_api_url },
                dev: env.dev.unwrap_or(config.dev),
                log_dir: resolve_log_dir_overlay(
                    Some(config.log_dir),
                    env.log_dir,
                ),
            })
        }
    }
}

fn parse_url(value: &str) -> Result<Url, ConfigError> {
    Url::parse(value).map_err(|err| ConfigError::InvalidRemoteApiUrl(err.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn embedded_file() -> TauriConfig {
        TauriConfig {
            deployment: DeploymentMode::Embedded {
                database_url: "file.db".into(),
                object_store_dir: PathBuf::from("objects/"),
            },
            dev: false,
            log_dir: default_log_dir(),
        }
    }

    fn remote_file() -> TauriConfig {
        TauriConfig {
            deployment: DeploymentMode::Remote {
                remote_api_url: Url::parse("http://127.0.0.1:3000/api").unwrap(),
            },
            dev: true,
            log_dir: default_log_dir(),
        }
    }

    #[test_log::test]
    fn deserializes_embedded_file() {
        let config = TauriConfig::from(
            serde_json::from_str::<TauriFileConfig>(
                r#"{
                "mode": "embedded",
                "databaseUrl": "yams.db",
                "objectStoreDir": "objects/",
                "dev": true
            }"#,
            )
            .unwrap(),
        );
        assert_eq!(
            config,
            TauriConfig {
                deployment: DeploymentMode::Embedded {
                    database_url: "yams.db".into(),
                    object_store_dir: PathBuf::from("objects/"),
                },
                dev: true,
                log_dir: default_log_dir(),
            }
        );
    }

    #[test_log::test]
    fn deserializes_remote_file() {
        let config = TauriConfig::from(
            serde_json::from_str::<TauriFileConfig>(
                r#"{
                "mode": "remote",
                "remoteApiUrl": "http://127.0.0.1:3000/api",
                "dev": true
            }"#,
            )
            .unwrap(),
        );
        assert_eq!(config, remote_file());
    }

    #[test_log::test]
    fn overlays_embedded_paths() {
        let resolved = overlay(
            embedded_file(),
            EnvOverlay {
                database_url: Some("env.db".into()),
                ..EnvOverlay::default()
            },
        )
        .unwrap();
        match resolved.deployment {
            DeploymentMode::Embedded { database_url, .. } => {
                assert_eq!(database_url, "env.db");
            }
            DeploymentMode::Remote { .. } => panic!("expected embedded"),
        }
    }

    #[test_log::test]
    fn switching_to_remote_requires_url() {
        let err = overlay(
            embedded_file(),
            EnvOverlay {
                mode: Some(ModeKind::Remote),
                ..EnvOverlay::default()
            },
        )
        .unwrap_err();
        assert!(matches!(err, ConfigError::RemoteEnvIncomplete));
    }

    #[test_log::test]
    fn switching_to_remote_with_url() {
        let resolved = overlay(
            embedded_file(),
            EnvOverlay {
                mode: Some(ModeKind::Remote),
                remote_api_url: Some("http://127.0.0.1:3000/api".into()),
                ..EnvOverlay::default()
            },
        )
        .unwrap();
        match resolved.deployment {
            DeploymentMode::Remote { remote_api_url } => {
                assert_eq!(remote_api_url.as_str(), "http://127.0.0.1:3000/api");
            }
            DeploymentMode::Embedded { .. } => panic!("expected remote"),
        }
    }

    #[test_log::test]
    fn remote_mode_rejects_embedded_env() {
        let err = overlay(
            remote_file(),
            EnvOverlay {
                database_url: Some("env.db".into()),
                ..EnvOverlay::default()
            },
        )
        .unwrap_err();
        assert!(matches!(err, ConfigError::EmbeddedEnvInRemoteMode));
    }

    #[test_log::test]
    fn embedded_mode_rejects_remote_env() {
        let err = overlay(
            embedded_file(),
            EnvOverlay {
                remote_api_url: Some("http://127.0.0.1:3000/api".into()),
                ..EnvOverlay::default()
            },
        )
        .unwrap_err();
        assert!(matches!(err, ConfigError::RemoteEnvInEmbeddedMode));
    }

    #[test_log::test]
    fn overlays_dev_flag() {
        let resolved = overlay(
            embedded_file(),
            EnvOverlay {
                dev: Some(true),
                ..EnvOverlay::default()
            },
        )
        .unwrap();
        assert!(resolved.dev);
    }

    #[test_log::test]
    fn deserializes_log_dir_from_file() {
        let config = TauriConfig::from(
            serde_json::from_str::<TauriFileConfig>(
                r#"{
                "mode": "embedded",
                "databaseUrl": "yams.db",
                "objectStoreDir": "objects/",
                "logDir": "/var/log/yams"
            }"#,
            )
            .unwrap(),
        );
        assert_eq!(config.log_dir, PathBuf::from("/var/log/yams"));
    }

    #[test_log::test]
    fn overlays_log_dir_from_env() {
        let resolved = overlay(
            embedded_file(),
            EnvOverlay {
                log_dir: Some(PathBuf::from("/tmp/yams-logs")),
                ..EnvOverlay::default()
            },
        )
        .unwrap();
        assert_eq!(resolved.log_dir, PathBuf::from("/tmp/yams-logs"));
    }

    #[test_log::test]
    fn frontend_dto_omits_backend_paths() {
        assert_eq!(
            embedded_file().frontend_dto(),
            FrontendConfigDto::Embedded { dev: false }
        );
        assert_eq!(
            remote_file().frontend_dto(),
            FrontendConfigDto::Remote {
                remote_api_url: "http://127.0.0.1:3000/api".into(),
                dev: true,
            }
        );
    }
}
