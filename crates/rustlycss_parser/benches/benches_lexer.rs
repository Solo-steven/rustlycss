use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rustlycss_parser::lexer::Lexer;
use rustlycss_types::token::*;
use rustlycss_types::config::GeneralConfig;

const TINT_FILE_STR: &str = include_str!("../../../assets/bootstrap-rebot.css");
const BIG_FILE_STR: &str =  include_str!("../../../assets/bootstrap.css");
const HUGE_FILE_STR: &str = include_str!("../../../assets/tailwind-dark.css");

fn criterion_benchmark(c: &mut Criterion) {
    let enable_sourcemap = GeneralConfig::from(true, true);
    c.bench_with_input(
        BenchmarkId::new("rustlycss tokenize tiny file (74kb)", "string of boostrap css"), 
        &TINT_FILE_STR, 
        |b, file| {
            b.iter(|| { 
                let mut lexer = Lexer::new(*file, &enable_sourcemap);
                loop {
                    match  lexer.next_token(){
                        Token::EOF => break ,
                        _ => {

                        }
                    }
                }
            })
        }
    );
    c.bench_with_input(
        BenchmarkId::new("rustlycss tokenize bigger file (201kb)", "string of boostrap css"), 
        &BIG_FILE_STR, 
        |b, file| {
            b.iter(|| { 
                let mut lexer = Lexer::new(*file, &enable_sourcemap);
                loop {
                    match  lexer.next_token(){
                        Token::EOF => break ,
                        _ => {

                        }
                    }
                }
            })
        }
    );
    c.bench_with_input(
        BenchmarkId::new("rustlycss tokenize huge file (5.8mb)", "string of boostrap css"), 
        &HUGE_FILE_STR, 
        |b, file| {
            b.iter(|| { 
                let mut lexer = Lexer::new(*file, &enable_sourcemap);
                loop {
                    match  lexer.next_token(){
                        Token::EOF => break ,
                        _ => {

                        }
                    }
                }
            })
        }
    );

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);