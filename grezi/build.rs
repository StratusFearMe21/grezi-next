use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use case::CaseExt;

fn create_ts_kinds(lang: tree_sitter::Language, file_name: &str, parser: &str) {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let mut out_enum = BufWriter::new(File::create(Path::new(&out_dir).join(file_name)).unwrap());

    out_enum
        .write_all(b"#[repr(u16)] #[derive(Default,FromPrimitive,Debug)] pub enum NodeKind {")
        .unwrap();

    for id in 0..lang.node_kind_count() as u16 {
        if lang.node_kind_is_named(id) && lang.node_kind_is_visible(id) {
            let kind = lang.node_kind_for_id(id).unwrap().to_camel();
            out_enum
                .write_fmt(format_args!("{}={},", kind, id))
                .unwrap();
        }
    }

    out_enum.write_all(b"#[default] Invalid}").unwrap();

    out_enum
        .write_all(b"#[repr(u16)] #[derive(Default,FromPrimitive,Debug)] pub enum FieldName {")
        .unwrap();

    for id in 0..lang.field_count() as u16 {
        if let Some(name) = lang.field_name_for_id(id) {
            out_enum
                .write_fmt(format_args!("{}={},", name.to_camel(), id))
                .unwrap();
        }
    }

    out_enum.write_all(b"#[default] Invalid}").unwrap();
    println!("cargo:rerun-if-changed={}", parser);
}

fn main() {
    create_ts_kinds(
        tree_sitter_grz::language(),
        "kinds.rs",
        "../tree-sitter-grz/src/parser.c",
    );
    create_ts_kinds(
        tree_sitter_ntbib::language(),
        "kinds_ntbib.rs",
        "../tree-sitter-ntbib/src/parser.c",
    );
}
