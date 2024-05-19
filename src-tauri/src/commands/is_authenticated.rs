use std::sync::Mutex;

use tauri::State;

use crate::{AppState, Error};

#[tauri::command]
pub fn is_authenticated(state_mutex: State<'_, Mutex<AppState>>) -> Result<bool, Error> {
    let state = state_mutex.lock()?;

    Ok(state.authenticated)
}
