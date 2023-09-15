use std::env;
use dotenv::dotenv;
use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let connection_string: String = env::var("DATABASE_URL").unwrap();

    // Connect to the database.
    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(())
}