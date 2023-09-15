mod utils;

use tokio_postgres::{NoTls, Error};
use crate::utils::crud;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = crud::establish_connection().await?;
    //crud::create_person(&client, "Jhon", "Doe", 20).await?;

    let persons= crud::read_persons(&client).await?;
    for person in persons {
        println!("Name: {}", person.name);
        println!("Last Name: {}", person.last_name);
        println!("Age: {}", person.age);
        println!();
    }

    Ok(())
}