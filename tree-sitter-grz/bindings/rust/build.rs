use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use case::CaseExt;

fn main() {
    let src_dir = std::path::Path::new("src");

    let mut c_config = cc::Build::new();
    c_config.std("c11").include(src_dir);

    #[cfg(target_env = "msvc")]
    c_config.flag("-utf-8");

    let parser_path = src_dir.join("parser.c");
    c_config.file(&parser_path);
    println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());

    let scanner_path = src_dir.join("scanner.c");
    if scanner_path.exists() {
        c_config.file(&scanner_path);
        println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());
    }

    c_config.compile("tree-sitter-grz");

    const SRC: &str = include_str!("../../src/parser.c");

    let mut src_lines = SRC.lines();

    for line in src_lines.by_ref() {
        if line == "enum ts_symbol_identifiers {" {
            break;
        }
    }

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let mut node_kinds_enum =
        BufWriter::new(File::create(Path::new(&out_dir).join("node_kinds.rs")).unwrap());
    node_kinds_enum
        .write_all(
            b"#[repr(u16)] #[derive(Default,num_enum::FromPrimitive,Debug,PartialEq,Eq,Clone,Copy)] pub enum NodeKind {",
        )
        .unwrap();

    for line in src_lines.by_ref() {
        if line == "};" {
            break;
        }

        let node = line.split_once('=').unwrap();
        let kind = node.0.trim().to_camel();
        let id = node.1.trim();
        node_kinds_enum
            .write_fmt(format_args!("{}={}", kind, id))
            .unwrap();
    }
    node_kinds_enum.write_all(b"#[default] Invalid}").unwrap();
    node_kinds_enum.flush().unwrap();

    for line in src_lines.by_ref() {
        if line == "enum ts_field_identifiers {" {
            break;
        }
    }

    let mut field_kinds_enum =
        BufWriter::new(File::create(Path::new(&out_dir).join("field_kinds.rs")).unwrap());
    field_kinds_enum
    .write_all(b"#[repr(u16)] #[derive(Default,num_enum::FromPrimitive,Debug,PartialEq,Eq,Clone,Copy)] pub enum FieldName {")
    .unwrap();

    for line in src_lines.by_ref() {
        if line == "};" {
            break;
        }

        let node = line.split_once('=').unwrap();
        let kind = node.0.trim().to_camel();
        let id = node.1.trim();
        field_kinds_enum
            .write_fmt(format_args!("{}={}", kind, id))
            .unwrap();
    }
    field_kinds_enum.write_all(b"#[default] Invalid}").unwrap();
    field_kinds_enum.flush().unwrap();
}
