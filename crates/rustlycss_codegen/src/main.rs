use rustlycss_parser::parser::Parser;
use rustlycss_codegen::Generator;
use rustlycss_types::config::GeneralConfig;
use sourcemap::{SourceMapBuilder};

pub mod source_map;

use std::fs::File;
use std::io::Write;

fn main() {
    let code = include_str!("../test.scss");
    let config = GeneralConfig::from(false, true);
    let mut parser = Parser::new(code, &config);
    let ast_root = parser.parse();
    println!("ast: {:?}", ast_root);
    let mut codegen = Generator::new(&config);
    codegen.generate(&ast_root);
    println!("output {:?}", codegen.output);
    let mut builder = SourceMapBuilder::new(None);
    let src_id =  builder.add_source("test.scss");

    for loc in &codegen.mapping {
        let dst = loc.1.clone();
        let src = loc.0.clone();
        println!("src ({:?}, {:?}) | dst ({:?}, {:?})", src.row, src.col, dst.row, dst.col);
        builder.add_raw(dst.row as u32, dst.col as u32, src.row as u32, src.col as u32, Some(src_id), None);
    }
    let srcmap = builder.into_sourcemap();
    let srcmap_output_file = File::create("./test.css.map").unwrap();
    srcmap.to_writer(srcmap_output_file).unwrap();
    let mut css_output_file = File::create("./test.css").unwrap();
    write!(css_output_file,"{}",codegen.output.as_str()).unwrap();
}