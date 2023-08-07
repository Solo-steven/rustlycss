use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rustlycss_parser::parser::Parser;
use rustlycss_types::config::GeneralConfig;

const TINT_FILE_STR: &str = include_str!("../../../assets/bootstrap-rebot.css");
const BIG_FILE_STR: &str =  include_str!("../../../assets/bootstrap.css");
const HUGE_FILE_STR: &str = include_str!("../../../assets/tailwind-dark.css");

fn criterion_benchmark(c: &mut Criterion) {
    let disable_sourcemap =  GeneralConfig::from(true, false);
    c.bench_with_input(
        BenchmarkId::new("rustlycss parse tiny file (74kb)", "string of boostrap css"), 
        &TINT_FILE_STR, 
        |b, file| {
            b.iter(|| { 
                let mut parser = Parser::new(file, &disable_sourcemap);
                parser.parse();
            })
        }
    );
    c.bench_with_input(
        BenchmarkId::new("rustlycss parse bigger file (201kb)", "string of boostrap css"), 
        &BIG_FILE_STR, 
        |b, file| {
            b.iter(|| { 
                let mut parser = Parser::new(file, &disable_sourcemap);
                parser.parse();
            })
        }
    );
    c.bench_with_input(
        BenchmarkId::new("rustlycss parse huge file (5.8mb)", "string of boostrap css"), 
        &HUGE_FILE_STR, 
        |b, file| {
            b.iter(|| { 
                let mut parser = Parser::new(file, &disable_sourcemap);
                parser.parse();
            })
        }
    );

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);