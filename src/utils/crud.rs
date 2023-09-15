use std::env;
use dotenv::dotenv;
use tokio_postgres::{NoTls, Client, Error};

pub async fn establish_connection() -> Result<Client, Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap();
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("!!!ERROR!!! Connection error: {}", e);
        }
    });

    Ok(client)
}

pub async fn create_person(client: &Client, name: &str, last_name: &str, age: i32) -> Result<(), Error> {
    client.execute("INSERT INTO persons (name, last_name, age) VALUES ($1, $2, $3)",&[&name, &last_name, &age],).await?;
    Ok(())
}

