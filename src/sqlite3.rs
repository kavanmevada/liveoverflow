use openssl::sha;
use rusqlite::{params, Connection, Row};

pub fn connection() -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open("database.db")?;

    conn.execute(
        "create table if not exists person (
        id integer primary key,
        username text not null unique,
        email text not null unique,
        password text not null
      )",
        [],
    )?;

    conn.execute(
        "create table if not exists sessions (
          id integer primary key,
          username text not null,
          session text not null unique
      )",
        [],
    )?;

    Ok(conn)
}

use crate::models::{Person, Session};

#[derive(Debug)]
pub enum Registration {
    Success(Person),
    Failed(RegistrationError),
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum RegistrationError {
    UsernameExist,
    MultipleAccounts,
    InvalidPassword,
    DatabaseError(String),
}

#[derive(Debug)]
pub enum LoginError {
    UsernameNotExist,
    InvalidPassword,
    TokenInvalid,
    DatabaseError(String),
}

#[derive(Debug)]
pub enum Login {
    Success(Person),
    Failed(LoginError),
}

pub fn register(con: &Connection, mut person: Person) -> Registration {
    person.password = encode!(sha::sha256(person.password.as_bytes()));

    match &con.execute(
        "INSERT INTO person (username, email, password)
        VALUES (:username, :email, :password)",
        &[
            (":username", &person.username),
            (":email", &person.email),
            (":password", &person.password),
        ],
    ) {
        Ok(id) => {
            person.id = *id;
            Registration::Success(person)
        }
        Err(e) => {
            dbg!(format!("UNIQUE constraint failed: {}", e));
            Registration::Failed(match &*format!("UNIQUE constraint failed: {}", e) {
                "person.email" => RegistrationError::MultipleAccounts,
                "person.username" => RegistrationError::UsernameExist,
                "person.password" => RegistrationError::InvalidPassword,
                _ => RegistrationError::DatabaseError(e.to_string()),
            })
        }
    }
}


#[derive(Debug)]
pub enum LoginType<'a> {
    Session(&'a str),              /* sessionid */
    Traditional(&'a str, &'a str), /* (username, password) */
}

pub fn login<'a>(con: &Connection, ltype: LoginType<'a>) -> Login {
    match ltype {
        LoginType::Session(sessionid) => {
            if let Some(mut person) =
                session(con, ("session", &sessionid)).and_then(|s| person(con, ("username", &s.username)))
            {
                person.sessionid = sessionid.to_string();
                Login::Success(person)
            } else {
                Login::Failed(LoginError::TokenInvalid)
            }
        }
        LoginType::Traditional(username, pass) => {
            let sessionid = encode!(sha::sha256(
                username
                    .bytes()
                    // .chain(browser.bytes())
                    // .chain(host.bytes())
                    .collect::<Vec<u8>>()
                    .as_slice()
            ));

            let password = encode!(sha::sha256(pass.as_bytes()));

            if let Some(mut person) = person(con, ("username", username)) {
                if person.password == password {
                    match con.execute(
                        "insert or replace into sessions (username, session) values (?1, ?2)",
                        params![username, sessionid],
                    ) {
                        Ok(_id) => {
                            person.sessionid = sessionid;
                            Login::Success(person)
                        },
                        Err(e) => Login::Failed(LoginError::DatabaseError(e.to_string())),
                    }
                } else {
                    Login::Failed(LoginError::InvalidPassword)
                }
            } else {
                Login::Failed(LoginError::UsernameNotExist)
            }
        }
    }
}

fn session(conn: &Connection, arg: (&str, &str)) -> Option<Session> {
    let mut stmt = conn
        .prepare(&format!("SELECT * FROM sessions WHERE {0} = :{0}", arg.0))
        .ok()?;
    let mut rows = stmt
        .query_and_then(
            &[(&*format!(":{}", arg.0), arg.1)],
            |row: &Row<'_>| -> Result<Session, rusqlite::Error> {
                Ok(Session {
                    id: row.get(0)?,
                    username: row.get(1)?,
                    sessionid: row.get(2)?,
                })
            },
        )
        .ok()?;

    rows.next()?.ok()
}


pub fn person(conn: &Connection, arg: (&str, &str)) -> Option<Person> {
    let mut stmt = conn
        .prepare(&format!("SELECT * FROM person WHERE {0} = :{0}", arg.0))
        .ok()?;
    let mut rows = stmt
        .query_and_then(
            &[(&*format!(":{}", arg.0), arg.1)],
            |row: &Row<'_>| -> Result<Person, rusqlite::Error> {
                Ok(Person {
                    id: row.get(0)?,
                    username: row.get(1)?,
                    email: row.get(2)?,
                    password: row.get(3)?,
                    sessionid: "null".to_string(),
                })
            },
        )
        .ok()?;

    rows.next()?.ok()
}


// pub fn person(conn: &Connection, username: &str) -> Option<Person> {
//     let mut stmt = conn
//         .prepare("select * from person where username=:username")
//         .ok()?;
//     let f = |row: &Row<'_>| -> Result<Person, rusqlite::Error> {
//         Ok(Person {
//             id: row.get(0)?,
//             username: row.get(1)?,
//             email: row.get(2)?,
//             password: row.get(3)?,
//             sessionid: "null".to_string(),
//         })
//     };

//     let mut rows = stmt.query_and_then(&[(":username", username)], f).ok()?;

//     rows.next()?.ok()
// }
