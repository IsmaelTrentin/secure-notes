use std::sync::Mutex;
use tauri::{Manager, State};

use crate::{AppState, Error};

#[tauri::command]
pub async fn logout(
    app: tauri::AppHandle,
    state_mutex: State<'_, Mutex<AppState>>,
) -> Result<(), Error> {
    let mut state = state_mutex.lock()?;
    state.authenticated = false;

    let _ = app.emit_all("logout", ());

    Ok(())
}
