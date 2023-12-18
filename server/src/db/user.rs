use futures_util::future::ok;
//server/src/db/user.rs
use tokio_rusqlite::{Connection as AsyncConnection, Error as TokioRusqliteError};
use std::sync::Arc;
use tokio::sync::Mutex;
use rusqlite::params;

pub struct User {
    pub name: String,
    pub password: String
}

pub async fn add_user(user:User) -> Result<(), TokioRusqliteError>{
    let conn = AsyncConnection::open("onegabo.db").await?;

    let data = conn.call( |conn| {
        conn.execute(
            "CREATE TABLE person (
                id    INTEGER PRIMARY KEY,
                name  TEXT NOT NULL,
                data  BLOB
            )",
            [],
        )?;
        Ok(())
    }).await?;

    Ok(())
}
