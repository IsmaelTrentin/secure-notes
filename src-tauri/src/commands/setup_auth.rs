use std::{fs::write, sync::Mutex};
use tauri::{Manager, State};

use crate::{AppState, Error};

#[tauri::command]
pub async fn setup_auth(
    master_pw: String,
    app: tauri::AppHandle,
    state_mutex: State<'_, Mutex<AppState>>,
) -> Result<(), Error> {
    let mut state = state_mutex.lock()?;
    state.needs_auth_setup = false;

    // TODO: encrypt
    let data_path = app
        .path_resolver()
        .app_data_dir()
        .expect("failed to resolve data path");
    let auth_file_path = data_path.join("master.auth");

    write(auth_file_path, &master_pw).expect("failed to write auth file");

    let _ = app.emit_all("auth_setup_ok", ());

    Ok(())
}
