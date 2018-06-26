#![feature(test)]

#[macro_use]
extern crate lazy_static;
extern crate test;
extern crate uap_rust;
extern crate woothee;

use test::{Bencher, black_box};
use uap_rust::parser as uap;
use woothee::parser as woo;

lazy_static! {
    static ref UAP_PARSER: uap::Parser = uap::Parser::new().unwrap();
    static ref WOO_PARSER: woo::Parser = woo::Parser::new();
}

fn b_uap() {
    black_box(UAP_PARSER.parse("Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 6.1; Trident/4.0)".to_string()));
    black_box(UAP_PARSER.parse("Twitterbot/1.0".to_string()));
    black_box(UAP_PARSER.parse("Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; Trident/5.0; Xbox)".to_string()));
}

fn b_woothee() {
    black_box(WOO_PARSER.parse("Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 6.1; Trident/4.0)"));
    black_box(WOO_PARSER.parse("Twitterbot/1.0"));
    black_box(WOO_PARSER.parse("Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; Trident/5.0; Xbox)"));
}

#[bench]
fn smoke_uap(b: &mut Bencher) {
    b.iter(b_uap);
}

#[bench]
fn smoke_woothee(b: &mut Bencher) {
    b.iter(b_woothee);
}

#[bench]
fn bench_stabilizer(b: &mut Bencher) {
    b.iter(|| "foobar".contains("oba"));
}
