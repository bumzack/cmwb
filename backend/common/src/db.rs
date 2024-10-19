use std::env;

use argon2::Config;
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use rand::prelude::*;

// is it any good? ¯\_(ツ)_/¯
// https://cloudmaker.dev/authenticate-api-users/
pub fn hash_password(password: &str) -> String {
    let mut rng = rand::thread_rng();
    let salt = rng.gen::<[u8; 32]>();
    let config = Config::default();
    let pw = argon2::hash_encoded(password.as_bytes(), &salt, &config).expect("should work");
    pw
}

// is it any good? ¯\_(ツ)_/¯
// https://cloudmaker.dev/authenticate-api-users/
pub fn verify_password(pw: String, password: String) -> bool {
    let res = argon2::verify_encoded(&pw, password.as_bytes()).expect("should work");
    res
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
