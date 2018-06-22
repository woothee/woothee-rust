#![feature(test)]

#[macro_use]
extern crate lazy_static;
extern crate test;
extern crate uap_rust;
extern crate woothee;

use test::Bencher;
use uap_rust::parser as uap;
use woothee::parser as woo;

lazy_static! {
    static ref UAP_PARSER: uap::Parser = uap::Parser::new().unwrap();
    static ref WOO_PARSER: woo::Parser = woo::Parser::new();
}

fn b_uap() {
    let _ = UAP_PARSER.parse("Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 6.1; Trident/4.0)".to_string());
    let _ = UAP_PARSER.parse("Twitterbot/1.0".to_string());
    let _ = UAP_PARSER.parse("Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; Trident/5.0; Xbox)".to_string());
}

fn b_woothee() {
    let _ = WOO_PARSER.parse("Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 6.1; Trident/4.0)");
    let _ = WOO_PARSER.parse("Twitterbot/1.0");
    let _ = WOO_PARSER.parse("Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; Trident/5.0; Xbox)");
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
