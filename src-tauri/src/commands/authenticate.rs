use std::{fs::read, sync::Mutex};
use tauri::{Manager, State};

use crate::{
    security::{decrypt, hash_key},
    AppState, Error,
};

#[tauri::command]
pub async fn authenticate(
    master_pw: String,
    app: tauri::AppHandle,
    state_mutex: State<'_, Mutex<AppState>>,
) -> Result<(), Error> {
    let mut state = state_mutex.lock()?;
    let auth_data = read(
        app.path_resolver()
            .app_data_dir()
            .unwrap()
            .join("master.auth"),
    )
    .expect("failed to read auth data");

    let key_hash_64 = hash_key(&master_pw);
    match decrypt(&key_hash_64, auth_data) {
        Ok(_) => {
            state.authenticated = true;

            let _ = app.emit_all("auth_ok", ());

            return Ok(());
        }
        Err(err) => {
            state.authenticated = false;
            return Err(err);
        }
    }
}
