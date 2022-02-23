use rusqlite::Connection;

pub fn connection() -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open("database.db")?;

    conn.execute(
        "create table if not exists person (
        id integer primary key,
        username text not null,
        email text not null,
        password text not null
      )",
        [],
    )?;

    conn.execute(
        "create table if not exists sessions (
          id integer primary key,
          session text not null,
          username text not null
      )",
        [],
    )?;

    Ok(conn)
}
