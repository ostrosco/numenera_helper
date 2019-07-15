use crate::error::NumeneraError;
use crate::item::*;
use crate::utils::{coin_flip, d10, d6};
use rusqlite::Connection;

#[derive(Debug)]
pub struct SalvageResult {
    shins: u8,
    parts: u8,
    oddity: Option<Oddity>,
    iotum: Option<Vec<Iotum>>,
    cyphers: Option<Vec<Cypher>>,
    artifact: Option<Artifact>,
}

pub fn random_salvage(
    conn: &Connection,
    item_level: u8,
) -> Result<SalvageResult, NumeneraError> {
    // Regardless of the random salvage roll, the party always get 1d10 shins.
    let roll = d6();
    let shins = d10();
    let coin_flip = coin_flip();
    decide_random_salvage(conn, roll, shins, coin_flip, item_level)
}

fn decide_random_salvage(
    conn: &Connection,
    roll: u8,
    shins: u8,
    coin_flip: u8,
    item_level: u8,
) -> Result<SalvageResult, NumeneraError> {
    let parts = item_level;
    let mut iotum = None;
    let mut oddity = None;
    let mut cyphers = None;
    let mut artifact = None;
    match roll {
        3 => {
            oddity = Some(get_oddity(conn)?);
        }
        4 => match coin_flip {
            1 => {
                iotum = Some(vec![get_iotum(conn, item_level)?]);
            }
            2 => {
                let mut cypher_vec = vec![];
                for _ in 1..=d6() {
                    cypher_vec.push(get_cypher(conn)?);
                }
                cyphers = Some(cypher_vec);
            }
            _ => (),
        },
        5 => match coin_flip {
            1 => {
                let mut iotum_vec = vec![];
                iotum_vec.push(get_iotum(conn, item_level)?);
                iotum_vec.push(get_iotum(conn, item_level)?);
                iotum = Some(iotum_vec);
            }
            2 => {
                artifact = Some(get_artifact(conn)?);
            }
            _ => (),
        },
        6 => {
            let mut iotum_vec = vec![];
            for _ in 0..3 {
                iotum_vec.push(get_iotum(conn, item_level)?);
            }
            iotum = Some(iotum_vec);
        }
        _ => (),
    }
    Ok(SalvageResult {
        shins,
        parts,
        iotum,
        oddity,
        cyphers,
        artifact,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_salvage_only_parts() {
        // A roll of 1 and 2 with the same inputs should produce the same
        // result, i.e. only shins and parts with no other perks.
        let shins = 10;
        let roll = 1;
        let item_level = 5;
        let coin_flip = 1;
        let conn = Connection::open("./numenera.db").unwrap();
        let res =
            decide_random_salvage(&conn, roll, shins, coin_flip, item_level)
                .unwrap();
        assert!(res.shins == 10);
        assert!(res.parts == 5);
        assert!(res.oddity.is_none());
        assert!(res.iotum.is_none());
        assert!(res.cyphers.is_none());
        assert!(res.artifact.is_none());

        let roll = 2;
        let res =
            decide_random_salvage(&conn, roll, shins, coin_flip, item_level)
                .unwrap();
        assert!(res.shins == shins);
        assert!(res.parts == item_level);
        assert!(res.oddity.is_none());
        assert!(res.iotum.is_none());
        assert!(res.cyphers.is_none());
        assert!(res.artifact.is_none());
    }

    #[test]
    fn test_random_salvage_oddity() {
        // A roll of 3 produces shins, parts, and a single oddity.
        let shins = 1;
        let roll = 3;
        let item_level = 7;
        let coin_flip = 1;
        let conn = Connection::open("./numenera.db").unwrap();
        let res =
            decide_random_salvage(&conn, roll, shins, coin_flip, item_level)
                .unwrap();
        assert!(res.shins == shins);
        assert!(res.parts == item_level);
        assert!(res.oddity.is_some());
        assert!(res.iotum.is_none());
        assert!(res.cyphers.is_none());
        assert!(res.artifact.is_none());
    }

    #[test]
    fn test_random_salvage_one_iotum() {
        // A roll of 4 can either result in an iotum or cyphers which is
        // decided by coin flip. In this case, we set the coin flip to give
        // an iotum.
        let shins = 2;
        let roll = 4;
        let item_level = 1;
        let coin_flip = 1;
        let conn = Connection::open("./numenera.db").unwrap();
        let res =
            decide_random_salvage(&conn, roll, shins, coin_flip, item_level)
                .unwrap();
        assert!(res.shins == shins);
        assert!(res.parts == item_level);
        assert!(res.oddity.is_none());
        assert!(res.iotum.is_some() && res.iotum.unwrap().len() == 1);
        assert!(res.cyphers.is_none());
        assert!(res.artifact.is_none());
    }

    #[test]
    fn test_random_salvage_cyphers() {
        // A roll of 4 can either result in an iotum or cyphers which is
        // decided by coin flip. In this case, we set the coin flip to give
        // 1d6 cyphers.
        let shins = 2;
        let roll = 4;
        let item_level = 2;
        let coin_flip = 2;
        let conn = Connection::open("./numenera.db").unwrap();
        let res =
            decide_random_salvage(&conn, roll, shins, coin_flip, item_level)
                .unwrap();
        assert!(res.shins == shins);
        assert!(res.parts == item_level);
        assert!(res.oddity.is_none());
        assert!(res.iotum.is_none());
        assert!(res.cyphers.is_some());
        let cyphers = res.cyphers.unwrap();
        assert!(cyphers.len() >= 1);
        assert!(cyphers.len() <= 6);
        assert!(res.artifact.is_none());
    }

    #[test]
    fn test_random_salvage_two_iotum() {
        // A roll of 5 can either result in either two iotum or one artifact.
        // In this case, we set the coin flip to give two iotum.
        let shins = 9;
        let roll = 5;
        let item_level = 6;
        let coin_flip = 1;
        let conn = Connection::open("./numenera.db").unwrap();
        let res =
            decide_random_salvage(&conn, roll, shins, coin_flip, item_level)
                .unwrap();
        assert!(res.shins == shins);
        assert!(res.parts == item_level);
        assert!(res.oddity.is_none());
        assert!(res.iotum.is_some() && res.iotum.unwrap().len() == 2);
        assert!(res.cyphers.is_none());
        assert!(res.artifact.is_none());
    }

    #[test]
    fn test_random_salvage_artifact() {
        // A roll of 5 can either result in either two iotum or one artifact.
        // In this case, we set the coin flip to give one artifact.
        let shins = 3;
        let roll = 5;
        let item_level = 8;
        let coin_flip = 2;
        let conn = Connection::open("./numenera.db").unwrap();
        let res =
            decide_random_salvage(&conn, roll, shins, coin_flip, item_level)
                .unwrap();
        assert!(res.shins == shins);
        assert!(res.parts == item_level);
        assert!(res.oddity.is_none());
        assert!(res.iotum.is_none());
        assert!(res.cyphers.is_none());
        assert!(res.artifact.is_some());
    }

    #[test]
    fn test_random_salvage_three_iotum() {
        // A roll of 6 awards three iotum.
        let shins = 9;
        let roll = 6;
        let item_level = 6;
        let coin_flip = 2;
        let conn = Connection::open("./numenera.db").unwrap();
        let res =
            decide_random_salvage(&conn, roll, shins, coin_flip, item_level)
                .unwrap();
        assert!(res.shins == shins);
        assert!(res.parts == item_level);
        assert!(res.oddity.is_none());
        assert!(res.iotum.is_some() && res.iotum.unwrap().len() == 3);
        assert!(res.cyphers.is_none());
        assert!(res.artifact.is_none());
    }

}
