use crate::error::NumeneraError;
use rand::{
    self,
    distributions::{Distribution, Uniform},
};
use regex::Regex;

pub fn coin_flip() -> u8 {
    let flip = Uniform::new_inclusive(1, 2);
    roll(flip)
}

pub fn d6() -> u8 {
    let d6 = Uniform::new_inclusive(1, 6);
    roll(d6)
}

pub fn d10() -> u8 {
    let d10 = Uniform::new_inclusive(1, 10);
    roll(d10)
}

pub fn d100() -> u8 {
    let d100 = Uniform::new_inclusive(1, 100);
    roll(d100)
}

pub fn roll_dice(dice_str: String) -> Result<u8, NumeneraError> {
    let dice_re = Regex::new("(\\d+)d(\\d+)(?:\\s*\\+\\s*(\\d+))?*")?;
    match dice_re.captures(&dice_str) {
        Some(captures) => {
            let num_rolls = str::parse::<u8>(
                captures
                    .get(1)
                    .ok_or(NumeneraError::DataFormatError)?
                    .as_str(),
            )?;
            let dice_sides = str::parse::<u8>(
                captures
                    .get(2)
                    .ok_or(NumeneraError::DataFormatError)?
                    .as_str(),
            )?;
            let roll_mod = match captures.get(3) {
                Some(val) => str::parse::<u8>(val.as_str())?,
                None => 0,
            };

            let dist = Uniform::new_inclusive(1, dice_sides);
            let mut rng = rand::thread_rng();
            let mut total = 0;

            for _ in 0..num_rolls {
                total += dist.sample(&mut rng);
            }
            Ok(total + roll_mod)
        }
        None => Ok(str::parse::<u8>(&dice_str)?),
    }
}

fn roll<T>(distribution: impl Distribution<T>) -> T {
    let mut rng = rand::thread_rng();
    distribution.sample(&mut rng)
}
