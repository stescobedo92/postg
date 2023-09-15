use std::env;
use dotenv::dotenv;
use tokio_postgres::{NoTls, Client, Error};
use crate::utils::models::Person;

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

pub async fn read_persons(client: &Client) -> Result<Vec<Person>, Error> {
    let stmt = client.prepare("SELECT id, name, last_name, age FROM persons").await?;
    let rows = client.query(&stmt, &[]).await?;

    let mut persons = vec![];
    for row in &rows {
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        let last_name: String = row.get(2);
        let age: i32 = row.get(3);
        persons.push(Person { id, name, last_name, age });
    }

    Ok(persons)
}

pub async fn update_person(client: &Client, id: i32, new_age: i32) -> Result<(), Error> {
    client.execute("UPDATE persons SET age = $1 WHERE id = $2",&[&new_age, &id]).await?;
    Ok(())
}

pub async fn delete_person(client: &Client, id: i32) -> Result<(), Error> {
    client.execute("DELETE FROM persons WHERE id = $1", &[&id]).await?;
    Ok(())
}

