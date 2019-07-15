use rusqlite::Connection;

mod error;
mod salvage;
mod item;
mod utils;

use crate::error::NumeneraError;

fn main() -> Result<(), NumeneraError> {
    let conn = Connection::open("./numenera.db")?;
    println!("{:#?}", salvage::random_salvage(&conn, 5)?);
    Ok(())
}
