use std::{borrow::Cow, collections::HashMap, fs::File, io::BufWriter, io::Write, path::Path};

use serde::Deserialize;

// JSON from <https://github.com/Bubblbu/paper-sizes>
const PAPER_SIZES: &str = include_str!("all_paper_sizes.json");

#[derive(Deserialize)]
pub struct PaperSize {
    points: [f32; 2],
}

fn main() {
    let paper_sizes: HashMap<Cow<'_, str>, HashMap<Cow<'_, str>, PaperSize>> =
        serde_json::from_str(PAPER_SIZES).unwrap();

    let path = Path::new(&std::env::var("OUT_DIR").unwrap()).join("paper_sizes.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let mut map = phf_codegen::Map::new();

    for (key, size) in paper_sizes.values().flatten() {
        map.entry(key.as_ref(), format!("{:?}", size.points));
    }

    write!(
        &mut file,
        "static PAPER_SIZES: phf::Map<&'static str, [f32; 2]> = {}",
        map.build()
    )
    .unwrap();
    write!(&mut file, ";\n").unwrap();
}
