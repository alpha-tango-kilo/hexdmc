use anyhow::{bail, Context, Result};
use std::env;

#[derive(Debug, Eq, PartialEq)]
struct Colour {
    floss: u16,
    name: &'static str,
    rgb: Rgb,
}

#[derive(Debug, Eq, PartialEq)]
struct Rgb(u8, u8, u8);

impl Rgb {
    fn from_hex(s: &str) -> Result<Rgb> {
        if s.starts_with('#') && s.len() == 7 {
            let r = u8::from_str_radix(&s[1..3], 16)?;
            let g = u8::from_str_radix(&s[3..5], 16)?;
            let b = u8::from_str_radix(&s[5..7], 16)?;
            Ok(Rgb(r, g, b))
        } else {
            bail!("not hex string")
        }
    }
}

fn main() -> Result<()> {
    // TODO: support many
    let arg = env::args()
        .nth(1)
        .expect("You were meant to provide a DMC or hex colour dumbass");
    match Rgb::from_hex(&arg) {
        Ok(_rgb) => {
            println!("Looking up colour based on hex value");
        }
        Err(_) => {
            let _floss = arg
                .parse::<u16>()
                .context("couldn't decipher input as floss or hex")?;
            println!("Looking up colour based on floss");
        }
    }
    Ok(())
}
