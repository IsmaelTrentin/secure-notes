// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use std::sync::Mutex;

use commands::authenticate::authenticate;
use commands::is_authenticated::is_authenticated;
use commands::logout::logout;
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("the mutex was poisoned")]
    PoisonError(String),
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

#[derive(Serialize)]
pub struct AppState {
    #[serde(skip_serializing)]
    pub authenticated: bool,
}
fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(AppState {
            authenticated: false,
        }))
        .invoke_handler(tauri::generate_handler![
            authenticate,
            is_authenticated,
            logout
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
