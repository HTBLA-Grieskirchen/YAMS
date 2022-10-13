use std::path::PathBuf;
use std::sync::Arc;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use url::Url;

fn dev_var_set() -> bool {
    let dev_var: &str = &std::env::var("YAMS_DEV").unwrap_or("0".to_string());
    match dev_var {
        "1" => true,
        _ => false
    }
}

fn project_dirs() -> ProjectDirs {
    ProjectDirs::from("at", "HTL Grieskirchen", "YAMS")
        .expect("unsupported OS")
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct YAMSFileConfig {
    #[serde(skip_serializing, skip_deserializing, default = "YAMSFileConfig::default_dirs")]
    pub dirs: ProjectDirs,

    #[serde(default)]
    pub remote_database_location: Option<String>,
}

impl YAMSFileConfig { fn default_dirs() -> ProjectDirs { YAMSFileConfig::default().dirs } }


#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct YAMSFrontendConfig {
    #[serde(skip_serializing)]
    file_config: Arc<YAMSFileConfig>,
    #[serde(skip_serializing)]
    backend_config: *const YAMSBackendConfig,

    pub remote_database_location: Option<Url>,
}

#[derive(Debug)]
pub struct YAMSBackendConfig {
    file_config: Arc<YAMSFileConfig>,
    frontend_config: *const YAMSFrontendConfig,

    is_in_dev: bool,
    pub local_database_location: String,
    pub uses_local_database: bool,
}


impl Default for YAMSFileConfig {
    fn default() -> Self {
        Self {
            dirs: project_dirs(),
            remote_database_location: None,
        }
    }
}


impl YAMSFileConfig {
    pub fn load() -> (YAMSBackendConfig, YAMSFrontendConfig) {
        let dirs = project_dirs();

        let file_location = if dev_var_set() {
            ["..", "yamsconfig.json"]
                .iter().collect::<PathBuf>()
        } else {
            dirs.config_dir().join("yamsconfig.json")
        };

        if let Ok(content) = std::fs::read_to_string(&file_location) {
            let file_config_result = serde_json::from_str::<YAMSFileConfig>(&content);
            if let Ok(file_config) = file_config_result {
                return file_config.to_configs();
            } else {
                println!("The config file {{{:?}}} is malformed.\
                 Resorting to default config.", file_location);
            }
        } else {
            println!("The config file {{{:?}}} does not exist.\
                 Resorting to default config.", file_location);
        }

        YAMSFileConfig::default_configs()
    }

    pub fn to_configs(self) -> (YAMSBackendConfig, YAMSFrontendConfig) {
        let arc_self = Arc::new(self);

        let mut backend_config: YAMSBackendConfig = Arc::clone(&arc_self).into();

        let mut frontend_config: YAMSFrontendConfig = Arc::clone(&arc_self).into();

        backend_config.frontend_config = &frontend_config;
        frontend_config.backend_config = &backend_config;

        (backend_config, frontend_config)
    }

    pub fn default_configs() -> (YAMSBackendConfig, YAMSFrontendConfig) {
        let arc_self = Arc::new(YAMSFileConfig::default());

        let mut backend_config = YAMSBackendConfig::default();
        backend_config.file_config = Arc::clone(&arc_self);

        let mut frontend_config = YAMSFrontendConfig::default();
        frontend_config.file_config = Arc::clone(&arc_self);

        backend_config.frontend_config = &frontend_config;
        frontend_config.backend_config = &backend_config;

        (backend_config, frontend_config)
    }
}

impl YAMSBackendConfig {
    pub fn as_frontend_config(&self) -> &YAMSFrontendConfig {
        unsafe {
            &*self.frontend_config
        }
    }

    pub fn as_file_config(&self) -> &YAMSFileConfig {
        &*self.file_config
    }

    pub fn dirs(&self) -> &ProjectDirs {
        &self.file_config.dirs
    }

    pub fn in_dev(&self) -> bool {
        self.is_in_dev
    }
}

impl From<Arc<YAMSFileConfig>> for YAMSBackendConfig {
    fn from(file_config: Arc<YAMSFileConfig>) -> Self {
        let in_dev = dev_var_set();

        let location = if in_dev {
            format!("file:{}", ["..", "..", "backend", "surreal_data"]
                .iter().collect::<PathBuf>().to_str().unwrap())
        } else {
            format!("file:{}", file_config.dirs.data_dir().join("surreal_data").to_str().unwrap())
        };

        let uses_local_database = (&file_config).remote_database_location.is_none();

        Self {
            file_config,
            frontend_config: std::ptr::null(),

            is_in_dev: in_dev,
            local_database_location: location,
            uses_local_database,
        }
    }
}

impl Default for YAMSBackendConfig {
    fn default() -> Self {
        let in_dev = dev_var_set();
        let dirs = project_dirs();

        let location = if in_dev {
            format!("file:{}", ["..", "..", "backend", "surreal_data"]
                .iter().collect::<PathBuf>().to_str().unwrap())
        } else {
            format!("file:{}", dirs.data_dir().join("surreal_data").to_str().unwrap())
        };

        Self {
            file_config: Arc::new(Default::default()),
            frontend_config: std::ptr::null(),

            is_in_dev: in_dev,
            local_database_location: location,
            uses_local_database: true,
        }
    }
}

impl YAMSFrontendConfig {
    pub fn as_backend_config(&self) -> &YAMSBackendConfig {
        unsafe {
            &*self.backend_config
        }
    }

    pub fn as_file_config(&self) -> &YAMSFileConfig {
        &*self.file_config
    }

    pub fn dirs(&self) -> &ProjectDirs {
        &self.file_config.dirs
    }
}

impl From<Arc<YAMSFileConfig>> for YAMSFrontendConfig {
    fn from(file_config: Arc<YAMSFileConfig>) -> Self {
        let remote_database_location = file_config.remote_database_location.clone().filter(|s| {
            !s.is_empty() && Url::parse(&s).is_ok()
        }).map(|s| Url::parse(&s).unwrap());

        Self {
            file_config,
            backend_config: std::ptr::null(),

            remote_database_location,
        }
    }
}

impl Default for YAMSFrontendConfig {
    fn default() -> Self {
        Self {
            file_config: Arc::new(Default::default()),
            backend_config: std::ptr::null(),

            remote_database_location: None,
        }
    }
}

unsafe impl Send for YAMSBackendConfig {}

unsafe impl Sync for YAMSBackendConfig {}

unsafe impl Send for YAMSFrontendConfig {}

unsafe impl Sync for YAMSFrontendConfig {}
