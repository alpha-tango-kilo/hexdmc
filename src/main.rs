use anyhow::{anyhow, bail, Context, Result};
use itertools::Itertools;
use owo_colors::{DynColors, OwoColorize};
use phf::{phf_map, Map};
use std::env;

type Rgb = [u8; 3];

const FULL_BLOCK: char = '█';

#[cfg(not(feature = "insulting"))]
mod error_strings {
    pub const INVALID_FLOSS: &str = "invalid floss";
    pub const NO_SUBCOMMAND: &str =
        "no subcommand provided. Refer to the README for usage instructions";
    pub const INVALID_SUBCOMMAND: &str = "invalid subcommand";
    pub const INVALID_HEX: &str = "invalid hex colour";
}

#[cfg(feature = "insulting")]
mod error_strings {
    pub const INVALID_FLOSS: &str = "that floss doesn't exist, you donkey";
    pub const NO_SUBCOMMAND: &str = "this grandma thinks they're too good to tell the program what to do; it should just read your mind";
    pub const INVALID_SUBCOMMAND: &str = "you ever heard of subcommands, you blithering idiot?! Read the README for crying out loud";
    pub const INVALID_HEX: &str = "it's meant to be a hexadecimal colour, it's not that bloody difficult!";
}

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
            None => Err(anyhow!(error_strings::INVALID_FLOSS)),
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

    fn similarity_iter(&self, rgb: Rgb) -> impl Iterator<Item = Colour> + '_ {
        let mut colours = self.colours;
        colours.sort_unstable_by_key(|a| a.diff(rgb));
        colours.into_iter()
    }
}

enum RgbMatch<'c> {
    Exact(&'c Colour),
    Approx(Vec<&'c Colour>),
}

#[derive(Copy, Clone)]
struct Colour {
    floss: &'static str,
    name: &'static str,
    r: u8,
    g: u8,
    b: u8,
}

impl Colour {
    fn to_rgb(self) -> Rgb {
        [self.r, self.g, self.b]
    }

    fn format_dmc(&self) -> String {
        format!(
            "{} ({}) {}",
            self.name,
            self.floss,
            FULL_BLOCK.color(self.owo())
        )
    }

    fn format_hex(&self) -> String {
        hex_from_rgb(self.to_rgb())
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

    fn owo(&self) -> DynColors {
        DynColors::Rgb(self.r, self.g, self.b)
    }
}

fn main() -> Result<()> {
    let subcommand =
        env::args().nth(1).context(error_strings::NO_SUBCOMMAND)?;

    let processing_fn = match subcommand.to_ascii_lowercase().as_str() {
        "hex" => match_hex_str,
        "dmc" => match_dmc_str,
        "diffdmc" => similar_dmc_str,
        _ => bail!(error_strings::INVALID_SUBCOMMAND),
    };

    env::args().skip(2).try_for_each(processing_fn)
}

fn match_hex_str<S: AsRef<str>>(hex_str: S) -> Result<()> {
    let rgb = rgb_from_hex(hex_str)?;
    let hex_str = hex_from_rgb(rgb); // standardises format
    let colour_match = ColourMap::lookup_rgb(&COLOUR_MAP, rgb);

    #[cfg(feature = "insulting")]
    match rgb {
        [0, 0, 0] => eprintln!("You're really asking about black? Really?"),
        [255, 255, 255] => eprintln!("You can't be the sharpest tool in the shed if you're asking about white"),
        _ => {}
    }

    use RgbMatch::*;
    match colour_match {
        Exact(c) => println!("{} -> {}", &hex_str, c.format_dmc()),
        Approx(cs) => {
            /*
            FIXME: When iter_intersperse goes stable, make itertools
             dependency conditional
            https://github.com/rust-lang/rust/issues/79524
            */
            #[allow(unstable_name_collisions)]
            let dmcs_string = cs
                .into_iter()
                .map(Colour::format_dmc)
                .intersperse(String::from(", or "))
                .collect::<String>();
            println!("{} ~> {}", &hex_str, dmcs_string);
        }
    }
    Ok(())
}

fn match_dmc_str<S: AsRef<str>>(dmc_str: S) -> Result<()> {
    let colour = ColourMap::lookup_floss(&COLOUR_MAP, dmc_str.as_ref())?;
    println!("{} -> {}", dmc_str.as_ref(), colour.format_hex());
    Ok(())
}

fn similar_dmc_str<S: AsRef<str>>(dmc_str: S) -> Result<()> {
    let colour = ColourMap::lookup_floss(&COLOUR_MAP, dmc_str.as_ref())?;
    ColourMap::similarity_iter(&COLOUR_MAP, colour.to_rgb())
        .take(5)
        .for_each(|c| println!("{} -> {}", c.format_dmc(), c.format_hex(),));
    // Separates out next cluster when multiple arguments are given
    println!();
    Ok(())
}

// e.g. "123455", "ab34ee", "AF1234"
fn rgb_from_hex<S: AsRef<str>>(hex_str: S) -> Result<Rgb> {
    let hex_str = hex_str.as_ref();
    let hex_digits = hex_str.strip_prefix('#').unwrap_or(hex_str);
    if hex_digits.len() == 6 {
        let r = u8::from_str_radix(&hex_digits[..2], 16)?;
        let g = u8::from_str_radix(&hex_digits[2..4], 16)?;
        let b = u8::from_str_radix(&hex_digits[4..], 16)?;
        Ok([r, g, b])
    } else {
        bail!(error_strings::INVALID_HEX)
    }
}

fn hex_from_rgb(rgb: Rgb) -> String {
    format!(
        "#{:02x}{:02x}{:02x} {}",
        rgb[0],
        rgb[1],
        rgb[2],
        FULL_BLOCK.color(rgb_owo(rgb))
    )
}

fn rgb_owo(rgb: Rgb) -> DynColors {
    DynColors::Rgb(rgb[0], rgb[1], rgb[2])
}

// Provides static COLOUR_MAP: ColourMap
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
