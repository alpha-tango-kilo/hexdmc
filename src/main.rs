use anyhow::{anyhow, bail, Context, Result};
use itertools::Itertools;
use phf::{phf_map, Map};
use std::env;

type Rgb = [u8; 3];

struct ColourMap<const N: usize> {
    colours: [Colour; N],
    by_floss: Map<&'static str, usize>,
    by_rgb: Map<Rgb, usize>,
}

impl<const N: usize> ColourMap<N> {
    // Note: will only not match if the floss is invalid, as all valid flosses
    // have a corresponding hex code
    fn lookup_floss(&self, floss: &str) -> Result<&Colour> {
        let floss = floss.to_ascii_lowercase();
        match self.by_floss.get(&floss) {
            Some(index) => Ok(&self.colours[*index]),
            None => Err(anyhow!("invalid floss")),
        }
    }

    fn lookup_rgb(&self, rgb: Rgb) -> RgbMatch {
        use RgbMatch::*;
        match self.by_rgb.get(&rgb) {
            Some(exact_index) => Exact(&self.colours[*exact_index]),
            None => {
                let mut min_diff = u16::MAX;
                let mut closest = Vec::new();
                self.colours.iter().for_each(|c| {
                    let diff = c.diff(rgb);
                    use std::cmp::Ordering::*;
                    match diff.cmp(&min_diff) {
                        Less => {
                            min_diff = diff;
                            closest = vec![c];
                        }
                        Equal => closest.push(c),
                        Greater => {}
                    }
                });
                Approx(closest)
            }
        }
    }
}

enum RgbMatch<'c> {
    Exact(&'c Colour),
    Approx(Vec<&'c Colour>),
}

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
        format!("#{:02x?}{:02x?}{:02x?}", self.r, self.g, self.b)
    }

    fn diff(&self, other: Rgb) -> u16 {
        let r_diff = self
            .r
            .checked_sub(other[0])
            .unwrap_or_else(|| other[0] - self.r);
        let g_diff = self
            .g
            .checked_sub(other[1])
            .unwrap_or_else(|| other[1] - self.g);
        let b_diff = self
            .b
            .checked_sub(other[2])
            .unwrap_or_else(|| other[2] - self.b);
        r_diff as u16 + g_diff as u16 + b_diff as u16
    }
}

fn main() -> Result<()> {
    let subcommand = env::args().nth(1).context("No subcommand provided")?;

    let processing_fn = match subcommand.to_ascii_lowercase().as_str() {
        "hex" => process_hex_str,
        "dmc" => process_dmc_str,
        _ => bail!("invalid subcommand"),
    };

    env::args().skip(2).try_for_each(processing_fn)
}

fn process_hex_str<S: AsRef<str>>(hex_str: S) -> Result<()> {
    let hex_str = hex_str.as_ref();
    let hex_str = hex_str.strip_prefix('#').unwrap_or(hex_str);
    let rgb = rgb_from_hex(hex_str)?;
    let colour: RgbMatch = COLOUR_MAP.lookup_rgb(rgb);

    use RgbMatch::*;
    match colour {
        Exact(c) => println!("#{} -> {}", hex_str, c.format_dmc()),
        Approx(cs) => {
            #[allow(unstable_name_collisions)]
            let dmcs_string = cs
                .into_iter()
                .map(Colour::format_dmc)
                .intersperse(String::from(", or "))
                .collect::<String>();
            println!("#{} ~> {}", hex_str, dmcs_string);
        }
    }
    Ok(())
}

fn process_dmc_str<S: AsRef<str>>(dmc_str: S) -> Result<()> {
    let colour: &Colour = COLOUR_MAP.lookup_floss(dmc_str.as_ref())?;
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

// Provides static COLOUR_MAP: ColourMap
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
