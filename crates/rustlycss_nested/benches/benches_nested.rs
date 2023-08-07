use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rustlycss_parser::parser::Parser;
use rustlycss_codegen::{Generator, source_map::build_source_map };
use rustlycss_nested::NestedVisitor;
use rustlycss_types::config::GeneralConfig;

const TINT_FILE_STR: &str = include_str!("../../../assets/nested-tiny.postcss");
const BIGGER_FILE_STR: &str = include_str!("../../../assets/nested-bigger.postcss");

fn criterion_benchmark(c: &mut Criterion) {
    let config = GeneralConfig::from(true, true);
    c.bench_with_input(
        BenchmarkId::new("rustlycss nested transform tiny file with generate and source-map", "string of nested css"), 
        &TINT_FILE_STR, 
        |b, file| {
            b.iter(|| { 
                let mut parser = Parser::new(file, &config);
                let mut root = parser.parse();
                let mut visitor = NestedVisitor::new();
                visitor.visit(&mut root);
                let mut codegen = Generator::new(&config);
                codegen.generate(&root);
                build_source_map(&codegen.mapping);
            })
        }
    );
    c.bench_with_input(
        BenchmarkId::new("rustlycss nested transform bigger file with generate and source-map", "string of nested css"), 
        &BIGGER_FILE_STR, 
        |b, file| {
            b.iter(|| { 
                let mut parser = Parser::new(file, &config);
                let mut root = parser.parse();
                let mut visitor = NestedVisitor::new();
                visitor.visit(&mut root);
                let mut codegen = Generator::new(&config);
                codegen.generate(&root);
                build_source_map(&codegen.mapping);
            })
        }
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
