use rusqlite::Connection;

use crate::Error;

fn setup_vaults(db: &Connection) -> Result<(), Error> {
    db.execute(
        "CREATE TABLE IF NOT EXISTS vaults(
  name TEXT,
  path TEXT UNIQUE,
  icon TEXT 
)",
        [],
    )?;

    Ok(())
}

pub fn setup_db(db: &Connection) -> Result<(), Error> {
    setup_vaults(db)?;

    Ok(())
}
