use std::{path::PathBuf, sync::Mutex};

use rusqlite::params;
use serde::Serialize;
use tauri::State;

use crate::{AppState, Error};

#[derive(Serialize)]
pub struct Vault {
    name: String,
    path: String,
    icon: Option<String>,
}

#[tauri::command]
pub fn get_vaults(state_mutex: State<'_, Mutex<AppState>>) -> Result<Vec<Vault>, Error> {
    let state = state_mutex.lock()?;
    let mut stmt = state.db.prepare("SELECT name, path, icon FROM vaults")?;
    let vaults_iter = stmt
        .query_map([], |row| {
            Ok(Vault {
                name: row.get(0)?,
                path: row.get(1)?,
                icon: row.get(2)?,
            })
        })?
        .map(|e| e.unwrap());

    let vaults: Vec<Vault> = vaults_iter.collect();

    Ok(vaults)
}

#[tauri::command]
pub fn add_vault(path: String, state_mutex: State<'_, Mutex<AppState>>) -> Result<Vault, Error> {
    let state = state_mutex.lock()?;
    let name = PathBuf::from(&path)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let mut stmt = state
        .db
        .prepare("INSERT INTO vaults (name, path) VALUES (?1, ?2)")?;
    stmt.execute(params![&name, &path])?;

    let vault = Vault {
        name,
        path,
        icon: None,
    };
    Ok(vault)
}
