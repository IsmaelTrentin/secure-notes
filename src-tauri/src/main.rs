// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod security;

use std::fs::{read_to_string, write};
use std::string::FromUtf8Error;
use std::sync::Mutex;

use commands::authenticate::{authenticate, is_authenticated};
use commands::logout::logout;
use commands::setup_auth::{needs_auth_setup, setup_auth};

use serde::{Deserialize, Serialize};
use tauri::Manager;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("the mutex was poisoned")]
    PoisonError(String),
    #[error("wrong password")]
    WrongPassword,
    #[error("unauthorized")]
    Unauthorized,
    #[error("bad utf8 data")]
    FromUtf8Error,
}
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(err: std::sync::PoisonError<T>) -> Self {
        Error::PoisonError(err.to_string())
    }
}
impl From<FromUtf8Error> for Error {
    fn from(_err: FromUtf8Error) -> Self {
        Error::FromUtf8Error
    }
}

#[derive(Serialize)]
pub struct AppState {
    #[serde(skip_serializing)]
    pub authenticated: bool,
    pub conf: AppConfig,
    #[serde(skip_serializing)]
    pub needs_auth_setup: bool,

    #[serde(skip_serializing)]
    pub key_hash_64: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub version: String,
    pub vault_paths: Vec<String>,
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let conf_path = app
                .path_resolver()
                .app_config_dir()
                .expect("failed to resolve config path");
            if !conf_path.exists() {
                std::fs::create_dir_all(&conf_path).expect("failed to create config path");
            }

            let conf;
            let conf_file_path = conf_path.join("config.toml");
            if !conf_file_path.exists() {
                let default_config = "version = \"0.0.1\"\nvault_paths = []\n";
                conf = toml::from_str(&default_config).expect("bad default config");
                write(conf_file_path, &default_config).expect("could not create conf file");
                println!("config file created");
            } else {
                let config_data =
                    read_to_string(conf_file_path).expect("failed to read config file");
                conf = toml::from_str(&config_data).expect("bad config");
                println!("read config file ok");
            }

            let data_path = app
                .path_resolver()
                .app_data_dir()
                .expect("failed to resolve data path");
            if !data_path.exists() {
                std::fs::create_dir_all(&data_path).expect("failed to create data path");
            }

            let auth_file_path = data_path.join("master.auth");
            let needs_auth_setup = !auth_file_path.exists();

            dbg!(&conf_path);
            dbg!(&data_path);
            dbg!(&conf);

            app.manage(Mutex::new(AppState {
                authenticated: false,
                conf,
                needs_auth_setup,
                key_hash_64: None,
            }));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            authenticate,
            is_authenticated,
            logout,
            needs_auth_setup,
            setup_auth
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
