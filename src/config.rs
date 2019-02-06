use dotenv::dotenv;
use std::env;

pub fn db_uri() -> String {

    dotenv().ok();

    let db_port = env::var("DB_HOST_PORT").expect("DB Host Port not set!");
    let db_host = env::var("DB_HOST").expect("DB Host not set!");
    let db_host = env::var("DB_HOST").expect("DB Host not set!");
    let db_user = env::var("POSTGRES_USER").expect("Postgres user not set!");
    let db_password = env::var("POSTGRES_PASSWORD").expect("Postgres user password not set!");

    format!("postgres://{}:{}@{}:{}",db_user, db_password, db_host, db_port)
}