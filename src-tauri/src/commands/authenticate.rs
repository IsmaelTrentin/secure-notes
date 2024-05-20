use std::sync::Mutex;
use tauri::{Manager, State};

use crate::{AppState, Error};

#[tauri::command]
pub async fn authenticate(
    app: tauri::AppHandle,
    state_mutex: State<'_, Mutex<AppState>>,
) -> Result<(), Error> {
    let mut state = state_mutex.lock()?;
    state.authenticated = true;

    let _ = app.emit_all("auth_ok", ());

    Ok(())
}
