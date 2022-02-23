use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct AppContext {
    pub connection: Connection,
}

impl AppContext {
    pub fn create(conn: Connection) -> Self {
        Self { connection: conn }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    // id: usize,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct Session {
    pub _session: String,
    pub username: String,
}
