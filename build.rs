use std::{
    fs::File,
    io::{BufReader, Write},
};

use serde::Deserialize;

mod hexdmc {
    use std::{env, fmt, path::Path};

    use super::*;

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

    pub fn generate_hexdmc_rs() {
        println!("cargo:rerun-if-changed=hexdmc.json");

        let file = File::open("hexdmc.json").unwrap();
        let reader = BufReader::new(file);
        let colours: Vec<Colour> = serde_json::from_reader(reader).unwrap();

        let out_dir = env::var_os("OUT_DIR").unwrap();
        let output = Path::new(&out_dir).join("hexdmc.rs");
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
            .try_for_each(|(n, c)| {
                writeln!(&mut output, "{:?} => {},", c.floss, n,)
            })
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
}

mod anchordmc {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct AnchorDmc {
        anchor: u16,
        dmc_name: String,
        human_name: String,
    }

    pub fn generate_anchordmc_rs() {
        println!("cargo:rerun-if-changed=anchordmc.json");

        let file = File::open("anchordmc.json").unwrap();
        let reader = BufReader::new(file);
        let info: Vec<AnchorDmc> = serde_json::from_reader(reader).unwrap();
        panic!("{:#?}", &info[0..4]);
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    hexdmc::generate_hexdmc_rs();
    anchordmc::generate_anchordmc_rs();
}
