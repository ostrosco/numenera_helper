use crate::utils::{d100, roll_dice};
use rusqlite::Result as SqlResult;
use rusqlite::{Connection, NO_PARAMS};

#[derive(Debug)]
pub struct Iotum {
    name: String,
    level: u8,
    units_salvaged: u8,
    value: u8,
}

#[derive(Debug)]
pub struct Oddity {
    table: String,
    book: String,
    page_number: u16,
    entry: u8,
}

#[derive(Debug)]
pub struct Cypher {
    name: String,
    level: u8,
    book: String,
    page_number: u16,
}

#[derive(Debug)]
pub struct Artifact {
    name: String,
    level: u8,
    book: String,
    page_number: u16,
}

pub fn get_iotum(conn: &Connection, item_level: u8) -> SqlResult<Iotum> {
    let roll = d100();
    conn.query_row(
        "SELECT name, level, units_salvaged, value FROM iotum
         WHERE min_roll <= ? AND max_roll >= ?",
        &[roll, roll],
        |row| {
            let name: String = row.get(0)?;
            let level: Option<u8> = row.get(1).ok();
            let units_salvaged: Option<String> = row.get(2).ok();
            let value: u8 = row.get(3)?;

            if name == "Plan seed" {
                Ok(Iotum {
                    name,
                    level: item_level,
                    units_salvaged: item_level,
                    value: value * item_level,
                })
            } else {
                let units_salvaged = roll_dice(&units_salvaged.unwrap())
                    .map_err(|_| rusqlite::Error::InvalidQuery)?;
                Ok(Iotum {
                    name,
                    level: level.unwrap(),
                    units_salvaged,
                    value,
                })
            }
        },
    )
}

pub fn get_oddity(conn: &Connection) -> SqlResult<Oddity> {
    let roll = d100();
    conn.query_row(
        "SELECT oddity_table, book, page_number FROM oddities
         ORDER BY RANDOM() LIMIT 1",
        NO_PARAMS,
        |row| {
            Ok(Oddity {
                table: row.get(0)?,
                book: row.get(1)?,
                page_number: row.get(2)?,
                entry: roll,
            })
        },
    )
}

pub fn get_cypher(conn: &Connection) -> SqlResult<Cypher> {
    conn.query_row(
        "SELECT name, level, book, page_number FROM cyphers 
         ORDER BY RANDOM() LIMIT 1",
        NO_PARAMS,
        |row| {
            let level_str: String = row.get(1)?;
            let level = roll_dice(&level_str)
                .map_err(|_| rusqlite::Error::InvalidQuery)?;
            Ok(Cypher {
                name: row.get(0)?,
                level,
                book: row.get(2)?,
                page_number: row.get(3)?,
            })
        },
    )
}

pub fn get_artifact(conn: &Connection) -> SqlResult<Artifact> {
    conn.query_row(
        "SELECT name, level, book, page_number FROM artifacts
         ORDER BY RANDOM() LIMIT 1",
        NO_PARAMS,
        |row| {
            let level_str: String = row.get(1)?;
            let level = roll_dice(&level_str)
                .map_err(|_| rusqlite::Error::InvalidQuery)?;
            Ok(Artifact {
                name: row.get(0)?,
                level,
                book: row.get(2)?,
                page_number: row.get(3)?,
            })
        },
    )
}
