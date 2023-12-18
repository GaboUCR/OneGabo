//server/src/db/user.rs
use tokio_rusqlite::{Connection as AsyncConnection, Error as TokioRusqliteError};
use rusqlite::params;

pub struct User {
    pub name: String,
    pub password: String
}

pub async fn add_user(user:User) -> Result<(), TokioRusqliteError>{
    let conn = AsyncConnection::open("onegabo.db").await?;

    let data = conn.call( move |conn| {
        conn.execute(
            "CREATE TABLE user (
                id    INTEGER PRIMARY KEY,
                name  TEXT NOT NULL UNIQUE,
                password TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "INSERT INTO  (name, data) VALUES (?1, ?2)",
            params![user.name, user.password],
        )?;

        Ok(())
    }).await?;

    Ok(())
}
