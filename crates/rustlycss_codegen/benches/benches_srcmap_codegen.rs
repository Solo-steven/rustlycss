use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rustlycss_codegen::Generator;
use rustlycss_codegen::source_map::build_source_map;
use rustlycss_parser::parser::Parser;
use rustlycss_types::config::GeneralConfig;

const TINT_FILE_STR: &str = include_str!("../../../assets/bootstrap-rebot.css");
const BIG_FILE_STR: &str =  include_str!("../../../assets/bootstrap.css");
const HUGE_FILE_STR: &str = include_str!("../../../assets/tailwind-dark.css");

fn criterion_benchmark(c: &mut Criterion) {
    let disable_sourcemap =  GeneralConfig::from(true, false);
    c.bench_with_input(
        BenchmarkId::new("rustlycss generate tiny file from ast(74kb) with source map", "string of boostrap css"), 
        &mut Parser::new(TINT_FILE_STR, &disable_sourcemap).parse(), 
        move |b, root| {
            let disable_sourcemap_2 =  GeneralConfig::from(true, true);
            b.iter(|| { 
               let mut generator = Generator::new(&disable_sourcemap_2);
               generator.generate(root);
               build_source_map(&generator.mapping);
            })
        }
    );
    c.bench_with_input(
        BenchmarkId::new("rustlycss generate bigger file from ast(201kb) with source map", "string of boostrap css"), 
        &mut Parser::new(BIG_FILE_STR, &disable_sourcemap).parse(), 
        |b, root| {
            let disable_sourcemap_2 =  GeneralConfig::from(true, true);
            b.iter(|| { 
               let mut generator = Generator::new(&disable_sourcemap_2);
               generator.generate(root);
               build_source_map(&generator.mapping);
            })
        }
    );
    c.bench_with_input(
        BenchmarkId::new("rustlycss generate huge file from ast(5.8mb) with source map", "string of boostrap css"), 
        &mut Parser::new(HUGE_FILE_STR, &disable_sourcemap).parse(), 
        |b, root| {
            let disable_sourcemap_2 =  GeneralConfig::from(true, true);
            b.iter(|| { 
               let mut generator = Generator::new(&disable_sourcemap_2);
               generator.generate(root);
               build_source_map(&generator.mapping);
            })
        }
    );

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);