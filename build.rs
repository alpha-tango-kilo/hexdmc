use std::{
    env, fmt,
    fs::File,
    io::{BufReader, Write},
    path::Path,
};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Colour {
    floss: StrOrNum,
    #[allow(dead_code)]
    name: String,
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum StrOrNum {
    S(String),
    N(u16),
}

// Custom impl hides the enum
impl fmt::Debug for StrOrNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use StrOrNum::*;
        match self {
            S(string) => write!(f, "{:?}", string.to_ascii_lowercase()),
            N(num) => write!(f, "\"{}\"", num),
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=colours.json");
    println!("cargo:rerun-if-changed=build.rs");
    let file = File::open("colours.json").unwrap();
    let reader = BufReader::new(file);
    let colours: Vec<Colour> = serde_json::from_reader(reader).unwrap();

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let output = Path::new(&out_dir).join("codegen.rs");
    let mut output = File::create(output).unwrap();
    writeln!(
        &mut output,
        r#"
        static COLOUR_MAP: ColourMap<{}> = ColourMap {{
            colours: {:#?},
            by_floss: ::phf::phf_map! {{
        "#,
        colours.len(),
        &colours,
    )
    .unwrap();

    // by_floss
    colours
        .iter()
        .enumerate()
        .try_for_each(
            |(n, c)| writeln!(&mut output, "{:?} => {},", c.floss, n,),
        )
        .unwrap();

    writeln!(&mut output, "}},\nby_rgb: ::phf::phf_map! {{").unwrap();

    // by_rgb
    colours
        .iter()
        .enumerate()
        .try_for_each(|(n, c)| {
            writeln!(&mut output, "[{}, {}, {}] => {},", c.r, c.g, c.b, n,)
        })
        .unwrap();

    writeln!(&mut output, "}},\n}};").unwrap();
}
