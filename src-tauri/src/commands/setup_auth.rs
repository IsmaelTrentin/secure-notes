use std::{fs::write, sync::Mutex};
use tauri::{Manager, State};

use crate::{
    security::{encrypt, hash_key, AUTH_CHECK_STR},
    AppState, Error,
};

#[tauri::command]
pub async fn setup_auth(
    master_pw: String,
    app: tauri::AppHandle,
    state_mutex: State<'_, Mutex<AppState>>,
) -> Result<(), Error> {
    let mut state = state_mutex.lock()?;
    state.needs_auth_setup = false;

    let data_path = app
        .path_resolver()
        .app_data_dir()
        .expect("failed to resolve data path");
    let auth_file_path = data_path.join("master.auth");

    let key_hash_64 = hash_key(&master_pw);
    let encrypted_data = encrypt(&key_hash_64, AUTH_CHECK_STR);
    write(auth_file_path, &encrypted_data).expect("failed to write auth file");

    let _ = app.emit_all("auth_setup_ok", ());

    Ok(())
}

#[tauri::command]
pub fn needs_auth_setup(state_mutex: State<'_, Mutex<AppState>>) -> Result<bool, Error> {
    let state = state_mutex.lock()?;

    Ok(state.needs_auth_setup)
}
