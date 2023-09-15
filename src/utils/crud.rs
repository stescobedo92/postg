use std::env;
use dotenv::dotenv;
use tokio_postgres::{NoTls, Client};

pub async fn establish_connection() -> Result<Client, Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("!!!ERROR!!! Connection error: {}", e);
        }
    });

    Ok(client)
}

