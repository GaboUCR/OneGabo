//server/src/db/user.rs
use tokio_rusqlite::{Connection as AsyncConnection, Error as TokioRusqliteError};
use rusqlite::params;
use bcrypt::{hash, verify, DEFAULT_COST};
pub struct User {
    pub name: String,
    pub password: String
}

pub async fn add_user(user: User) -> Result<(), TokioRusqliteError> {

    let conn = match AsyncConnection::open("./onegabo.db").await {
        Ok(conn) => {
            conn
        },
        Err(e) => {
            eprintln!("Error al abrir la base de datos: {:?}", e);
            return Err(e);
        }
    };

    let hashed_password = match hash(&user.password, DEFAULT_COST) {
        Ok(h) => {
            h
        },
        Err(e) => {
            eprintln!("Error al hashear la contraseña: {:?}", e);
            return Err(TokioRusqliteError::Other(Box::new(e)));
        }
    };

    let result = conn.call(move |conn| {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS user (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                password TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "INSERT INTO user (name, password) VALUES (?1, ?2)",
            params![user.name, hashed_password],
        )?;

        Ok(())
    }).await;

    match result {
        Ok(_) => {
            Ok(())
        },
        Err(e) => {
            eprintln!("Error durante las operaciones en la base de datos: {:?}", e);
            Err(e)
        }
    }
}

pub async fn verify_user_credential (user: User) -> Result<bool, TokioRusqliteError> {
    let conn = AsyncConnection::open("onegabo.db").await?;

    let result = conn.call(move |conn| {
        let mut stmt = conn.prepare("SELECT password FROM user WHERE name = ?1")?;
        let mut rows = stmt.query(params![&user.name])?;

        if let Some(row) = rows.next()? {
            let hashed_password: String = row.get(0)?;
            match verify(&user.password, &hashed_password) {
                Ok(valid) => Ok(valid),
                Err(e) => {
                    eprintln!("Error al verificar la contraseña: {:?}", e);
                    let other = TokioRusqliteError::Other(Box::new(e));
                    Err(other)
                }
            }
        } else {
            Ok(false)  // Usuario no encontrado
        }
    }).await?;

    Ok(result)
}