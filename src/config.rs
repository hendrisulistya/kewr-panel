use std::path::PathBuf;
use std::env;

#[derive(Clone)]
pub struct WorkingDirectoryConfig {
    pub base_dir: PathBuf,
    pub www_dir: PathBuf,
    pub logs_dir: PathBuf,
    pub config_dir: PathBuf,
    pub run_dir: PathBuf
}

impl WorkingDirectoryConfig {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        let is_development = env::var("ENV").unwrap_or_else(|_| String::from("development")) == "development";

        let base_dir = if is_development {
            PathBuf::from(env::var("DEV_BASE_DIR").unwrap_or_else(|_| String::from("./")))
        } else {
            PathBuf::from(env::var("BASE_DIR").unwrap_or_else(|_| String::from("/opt/kewr-panel")))
        };

        let www_dir = if is_development {
            PathBuf::from(env::var("DEV_WWW_DIR").unwrap_or_else(|_| String::from("./www")))
        } else {
            PathBuf::from(env::var("WWW_DIR").unwrap_or_else(|_| String::from("/var/lib/kewr-panel")))
        };

        let logs_dir = if is_development {
            PathBuf::from(env::var("DEV_LOG_DIR").unwrap_or_else(|_| String::from("./logs")))
        } else {
            PathBuf::from(env::var("LOG_DIR").unwrap_or_else(|_| String::from("/var/log/kewr-panel")))
        };

        let config_dir = if is_development {
            PathBuf::from(env::var("DEV_CONFIG_DIR").unwrap_or_else(|_| String::from("./config")))
        } else {
            PathBuf::from(env::var("CONFIG_DIR").unwrap_or_else(|_| String::from("/etc/kewr-panel")))
        };

        let run_dir = if is_development {
            PathBuf::from(env::var("DEV_RUNTIME_DIR").unwrap_or_else(|_| String::from("./run")))
        } else {
            PathBuf::from(env::var("RUNTIME_DIR").unwrap_or_else(|_| String::from("/var/run/kewr-panel")))
        };

        Self {
            base_dir,
            www_dir,
            logs_dir,
            config_dir,
            run_dir
        }
    }

    pub fn ensure_directories_exist(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.base_dir)?;
        std::fs::create_dir_all(&self.www_dir)?;
        std::fs::create_dir_all(&self.logs_dir)?;
        std::fs::create_dir_all(&self.config_dir)?;
        std::fs::create_dir_all(&self.run_dir)?;
        Ok(())
    }
}