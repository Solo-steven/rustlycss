use rustlycss_types::position::Position;
use sourcemap::SourceMapBuilder;

pub fn build_source_map(mapping: &Vec<(Position, Position)>) -> sourcemap::SourceMap {
    let mut builder = SourceMapBuilder::new(None);
    let src_id =  builder.add_source("test.scss");

    for loc in mapping {
        let dst_row = loc.1.row;
        let dst_col = loc.1.col;
        let src_row = loc.0.row;
        let src_col = loc.0.col;
        builder.add_raw(dst_row as u32, dst_col as u32, src_row as u32, src_col as u32, Some(src_id), None);
    }
    builder.into_sourcemap()
}