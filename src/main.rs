use clap::{value_t, App, Arg, SubCommand};
use rusqlite::Connection;

mod error;
mod item;
mod salvage;
mod utils;

use crate::error::NumeneraError;

fn main() -> Result<(), NumeneraError> {
    let conn = Connection::open("./numenera.db")?;

    let matches = App::new("Numenera Helper")
        .version("0.1")
        .author("Ostrosco")
        .about("Makes a Numenera campaign smoother")
        .subcommand(
            SubCommand::with_name("salvage")
                .about("Generate a random salvage result.")
                .arg(
                    Arg::with_name("level")
                        .short("l")
                        .required(true)
                        .help("Item level of the object being salvaged.")
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("loot")
                .arg(
                    Arg::with_name("cyphers")
                        .short("c")
                        .help("Generate a number of cyphers.")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("artifacts")
                        .short("a")
                        .help("Generate a number of artifacts.")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("oddities")
                        .short("o")
                        .help("Generate a number of oddities.")
                        .takes_value(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("salvage") {
        let level = value_t!(matches, "level", u8).unwrap();
        println!("Random salvage result:");
        println!("{:#?}", salvage::random_salvage(&conn, level)?);
    }

    if let Some(matches) = matches.subcommand_matches("loot") {
        let num_cyphers = value_t!(matches, "cyphers", u8).unwrap_or(0);
        let num_artifacts = value_t!(matches, "artifacts", u8).unwrap_or(0);
        let num_oddities = value_t!(matches, "oddities", u8).unwrap_or(0);

        for _ in 0..num_cyphers {
            println!("{:#?}", item::get_cypher(&conn)?);
        }
        for _ in 0..num_artifacts {
            println!("{:#?}", item::get_artifact(&conn)?);
        }
        for _ in 0..num_oddities {
            println!("{:#?}", item::get_oddity(&conn)?);
        }
    }
    Ok(())
}
