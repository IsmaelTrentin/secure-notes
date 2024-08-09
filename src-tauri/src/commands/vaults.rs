use std::fs::{self};
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

#[derive(Serialize)]
pub struct VaultEntry {
    pub path: String,
    pub filename: String,
    pub children: Vec<Box<VaultEntry>>,
}

impl VaultEntry {
    fn new(path: String, filename: String) -> VaultEntry {
        VaultEntry {
            path,
            filename,
            children: Vec::<Box<VaultEntry>>::new(),
        }
    }

    fn add_child<T>(&mut self, leaf: T) -> &mut Self
    where
        T: Into<VaultEntry>,
    {
        self.children.push(Box::new(leaf.into()));
        self
    }
}

fn get_dir_entries(dir_path: PathBuf) -> Result<VaultEntry, Error> {
    let elements_iter = fs::read_dir(&dir_path)?;
    let path = dir_path.to_string_lossy().to_string();
    let filename = dir_path.file_name().unwrap().to_string_lossy().to_string();
    let mut entries: VaultEntry = VaultEntry::new(path, filename);

    for element in elements_iter {
        match element.is_ok() {
            true => {
                let dir_entry = element.unwrap();
                let ft = dir_entry.file_type()?;

                if ft.is_dir() {
                    entries.add_child(get_dir_entries(dir_entry.path())?);
                } else {
                    entries.add_child(VaultEntry::new(
                        dir_entry.path().to_string_lossy().to_string(),
                        dir_entry.file_name().to_string_lossy().to_string(),
                    ));
                }
            }
            false => todo!(),
        }
    }

    Ok(entries)
}
#[tauri::command]
pub fn get_vault_entries(vault_path: String) -> Result<VaultEntry, Error> {
    let entries = get_dir_entries(PathBuf::from(vault_path))?;

    Ok(entries)
}
