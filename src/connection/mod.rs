use std::env;

use postgres;
use postgres::{Connection, SslMode};

pub fn db_conn() -> postgres::Connection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let conn = Connection::connect(&database_url[..], SslMode::None).unwrap();
    return conn;
}
