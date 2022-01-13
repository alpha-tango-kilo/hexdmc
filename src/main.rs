use anyhow::{anyhow, bail, Context, Result};
use codegen::COLOUR_MAP;
use phf::{phf_map, Map, PhfHash};
use phf_shared::PhfBorrow;
use std::env;
use std::hash::Hasher;
use std::sync::Arc;

mod codegen;

pub struct ColourMap {
    by_floss: Map<u16, Arc<Colour>>,
    by_rgb: Map<Rgb, Arc<Colour>>,
}

impl ColourMap {
    // Note: will only not match if the floss is invalid, as all valid flosses
    // have a corresponding hex code
    fn lookup_floss(&self, floss: u16) -> Result<&Colour> {
        match self.by_floss.get(&floss) {
            Some(colour) => Ok(colour.as_ref()),
            None => Err(anyhow!("invalid floss")),
        }
    }

    fn lookup_rgb(&self, rgb: Rgb) -> &Colour {
        match self.by_rgb.get(&rgb) {
            Some(exact) => exact.as_ref(),
            None => {
                println!("No direct match, approximating");
                todo!()
            }
        }
    }

    // FIXME: remove this once we have a build script
    pub const fn new() -> Self {
        ColourMap {
            by_floss: phf_map! {},
            by_rgb: phf_map! {},
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Colour {
    floss: u16,
    name: &'static str,
    rgb: Rgb,
}

impl Colour {
    fn format_dmc(&self) -> String {
        format!("{} ({})", self.name, self.floss)
    }

    fn format_hex(&self) -> String {
        self.rgb.format_hex()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Rgb(u8, u8, u8);

impl Rgb {
    // e.g. "123455", "ab34ee", "AF1234"
    fn from_hex(s: &str) -> Result<Rgb> {
        if s.len() == 6 {
            let r = u8::from_str_radix(&s[1..3], 16)?;
            let g = u8::from_str_radix(&s[3..5], 16)?;
            let b = u8::from_str_radix(&s[5..7], 16)?;
            Ok(Rgb(r, g, b))
        } else {
            bail!("not hex string")
        }
    }

    fn format_hex(&self) -> String {
        format!("#{:02x?}{:02x?}{:02x?}", self.0, self.1, self.2)
    }
}

impl PhfHash for Rgb {
    fn phf_hash<H: Hasher>(&self, state: &mut H) {
        let arr = [self.0, self.1, self.2];
        arr.phf_hash(state);
    }
}

impl PhfBorrow<Rgb> for Rgb {
    fn borrow(&self) -> &Rgb {
        self
    }
}

fn main() -> Result<()> {
    let subcommand = env::args().nth(1).context("No subcommand provided")?;

    let processing_fn = match subcommand.as_str() {
        "hex" => process_hex_str,
        "dmc" => process_dmc_str,
        _ => bail!("invalid subcommand"),
    };

    env::args().skip(2).try_for_each(processing_fn)
}

fn process_hex_str<S: AsRef<str>>(hex_str: S) -> Result<()> {
    let rgb = Rgb::from_hex(hex_str.as_ref())?;
    let colour = COLOUR_MAP.lookup_rgb(rgb);
    println!("{} -> {}", rgb.format_hex(), colour.format_dmc());
    Ok(())
}

fn process_dmc_str<S: AsRef<str>>(dmc_str: S) -> Result<()> {
    let floss = dmc_str.as_ref().parse::<u16>()?;
    let colour = COLOUR_MAP.lookup_floss(floss)?;
    println!("{} -> {}", floss, colour.format_hex());
    Ok(())
}
