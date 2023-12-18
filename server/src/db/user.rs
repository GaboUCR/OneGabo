//server/src/db/user.rs
use tokio_rusqlite::{Connection as AsyncConnection, Error as TokioRusqliteError};
use rusqlite::params;

pub struct User {
    pub name: String,
    pub password: String
}

pub async fn add_user(user: User) -> Result<(), TokioRusqliteError> {
    // Establecer conexión con la base de datos
    let conn = match AsyncConnection::open("onegabo.db").await {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Error al conectar con la base de datos: {:?}", e);
            return Err(e);
        },
    };

    // Ejecutar operaciones de base de datos
    let result = conn.call(move |conn| {
        // Crear tabla si no existe
        if let Err(e) = conn.execute(
            "CREATE TABLE IF NOT EXISTS user (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                password TEXT NOT NULL
            )",
            [],
        ) {
            eprintln!("Error al crear la tabla: {:?}", e);
            return Err(tokio_rusqlite::Error::Rusqlite(e));
        }

        // Insertar usuario
        match conn.execute(
            "INSERT INTO user (name, password) VALUES (?1, ?2)",
            params![user.name, user.password],
        ) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Error al insertar el usuario: {:?}", e);
                Err(tokio_rusqlite::Error::Rusqlite(e))
            },
        }
    }).await;

    // Manejar resultado de operaciones de base de datos
    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Error durante la operación de base de datos: {:?}", e);
            Err(e)
        },
    }
}

// pub async fn get_user(user:User) -> Result<(), TokioRusqliteError>{
//     let conn = AsyncConnection::open("onegabo.db").await?;

//     let data = conn.call( move |conn| {
//         conn.execute(
//             "CREATE TABLE user (
//                 id    INTEGER PRIMARY KEY,
//                 name  TEXT NOT NULL UNIQUE,
//                 password TEXT NOT NULL
//             )",
//             [],
//         )?;

//         conn.execute(
//             "INSERT INTO  (name, data) VALUES (?1, ?2)",
//             params![user.name, user.password],
//         )?;

//         Ok(())
//     }).await?;

//     Ok(())
// }