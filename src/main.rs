mod utils;

use tokio_postgres::{NoTls, Error};
use crate::utils::crud;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = crud::establish_connection().await?;
    crud::create_person(&client, "Jhon", "Doe", 20).await?;

    Ok(())
}