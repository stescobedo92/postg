mod utils;

use std::env;
use dotenv::dotenv;
use tokio_postgres::{NoTls, Error};
use crate::utils::crud;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = crud::establish_connection();

    Ok(())
}