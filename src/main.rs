use anyhow::{anyhow, bail, Context, Result};
use phf::{phf_map, Map};
use std::env;

type Rgb = [u8; 3];

#[derive(Debug)]
struct ColourMap<const N: usize> {
    colours: [Colour; N],
    by_floss: Map<&'static str, usize>,
    by_rgb: Map<Rgb, usize>,
}

impl<const N: usize> ColourMap<N> {
    // Note: will only not match if the floss is invalid, as all valid flosses
    // have a corresponding hex code
    fn lookup_floss(&self, floss: &str) -> Result<&Colour> {
        match self.by_floss.get(floss) {
            Some(index) => Ok(&self.colours[*index]),
            None => Err(anyhow!("invalid floss")),
        }
    }

    fn lookup_rgb(&self, rgb: Rgb) -> &Colour {
        match self.by_rgb.get(&rgb) {
            Some(exact_index) => &self.colours[*exact_index],
            None => {
                println!("No direct match, approximating");
                todo!()
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Colour {
    floss: &'static str,
    name: &'static str,
    r: u8,
    g: u8,
    b: u8,
}

impl Colour {
    fn format_dmc(&self) -> String {
        format!("{} ({})", self.name, self.floss)
    }

    fn format_hex(&self) -> String {
        rgb_format_hex(self.to_rgb())
    }

    fn to_rgb(&self) -> Rgb {
        [self.r, self.g, self.b]
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
    let rgb = rgb_from_hex(hex_str.as_ref())?;
    let colour = COLOUR_MAP.lookup_rgb(rgb);
    println!("{} -> {}", rgb_format_hex(rgb), colour.format_dmc());
    Ok(())
}

fn process_dmc_str<S: AsRef<str>>(dmc_str: S) -> Result<()> {
    let colour = COLOUR_MAP.lookup_floss(dmc_str.as_ref())?;
    println!("{} -> {}", dmc_str.as_ref(), colour.format_hex());
    Ok(())
}

// e.g. "123455", "ab34ee", "AF1234"
fn rgb_from_hex(s: &str) -> Result<Rgb> {
    if s.len() == 6 {
        let r = u8::from_str_radix(&s[..2], 16)?;
        let g = u8::from_str_radix(&s[2..4], 16)?;
        let b = u8::from_str_radix(&s[4..], 16)?;
        Ok([r, g, b])
    } else {
        bail!("not hex string")
    }
}

fn rgb_format_hex(rgb: Rgb) -> String {
    format!("#{:02x?}{:02x?}{:02x?}", rgb[0], rgb[1], rgb[2])
}

// Provides static COLOUR_MAP: ColourMap
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
