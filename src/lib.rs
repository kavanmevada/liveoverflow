macro_rules! encode {
    ($e:expr) => { $e.iter().map(|b| format!("{:02x}", b)).collect::<String>() }
}

// Expose modules
pub mod sqlite3;
pub mod ssl;

pub mod models;
pub mod services;
pub mod api;

// let sha256 = openssl::sha::sha256(person.password.as_bytes());
// let sha256_str = sha256
//     .iter()
//     .map(|b| format!("{:02x}", b))
//     .collect::<String>();
