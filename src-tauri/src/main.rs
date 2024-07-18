// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_config;
mod commands;
mod db;
mod security;

use std::string::FromUtf8Error;
use std::sync::Mutex;

use app_config::{ensure_config_dir, read_config, AppConfig};
use commands::add_vault;
use commands::authenticate;
use commands::get_config;
use commands::get_vaults;
use commands::is_authenticated;
use commands::logout;
use commands::needs_auth_setup;
use commands::reload_config;
use commands::setup_auth;

use db::setup_db;
use rusqlite::Connection;
use serde::Serialize;
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
    #[error("bad config")]
    BadConfig(String),
    #[error("SQL error")]
    SqlError(#[from] rusqlite::Error),
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
    pub conf: AppConfig,

    #[serde(skip_serializing)]
    pub db: Connection,

    #[serde(skip_serializing)]
    pub authenticated: bool,
    #[serde(skip_serializing)]
    pub needs_auth_setup: bool,
    #[serde(skip_serializing)]
    pub key_hash_64: Option<String>,
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let conf_path = ensure_config_dir(app);

            let data_path = app
                .path_resolver()
                .app_data_dir()
                .expect("failed to resolve data path");
            if !data_path.exists() {
                std::fs::create_dir_all(&data_path).expect("failed to create data path");
            }

            let mut conf = read_config(&conf_path.join("config.toml")).unwrap();
            let data_path = app
                .path_resolver()
                .app_data_dir()
                .expect("failed to resolve data path");
            if !data_path.exists() {
                std::fs::create_dir_all(&data_path).expect("failed to create data path");
            }

            conf.db_path = conf
                .db_path
                .replace("<data>", &data_path.display().to_string());

            let auth_file_path = data_path.join("master.auth");
            let needs_auth_setup = !auth_file_path.exists();

            dbg!(&conf_path);
            dbg!(&data_path);
            dbg!(&conf);

            let db = Connection::open(&conf.db_path)?;
            println!("running db init queries...");
            setup_db(&db)?;
            println!("db init done");

            app.manage(Mutex::new(AppState {
                conf,
                db,
                authenticated: false,
                needs_auth_setup,
                key_hash_64: None,
            }));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // auth
            authenticate,
            is_authenticated,
            needs_auth_setup,
            setup_auth,
            logout,
            // config
            get_config,
            reload_config,
            // vaults
            get_vaults,
            add_vault,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
