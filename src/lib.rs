//! # Woothee
//!
//! Woothee is a user-agent strings parser.
//!
//! ## Usage
//!
//! ```toml
//! [dependencies]
//! woothee = "*"
//! ```
//!
//! ```rust
//! extern crate woothee;
//!
//! use woothee::parse;
//!
//! fn main() {
//!     let result = parse("Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 6.1; Trident/4.0)");
//!     println!("{:?}", result);
//! }
//! ```
//!

extern crate regex;

pub mod woothee;
pub mod parser;
pub mod dataset;

use parser::{Parser, WootheeResult};

pub fn parse(agent: &str) -> Option<WootheeResult> {
    let parser = Parser::new();
    parser.parse(agent)
}

pub fn is_crawler(agent: &str) -> bool {
    if agent.is_empty() || agent == "-" {
        return false;
    }

    let parser = Parser::new();
    let mut result = WootheeResult::new();
    parser.try_crawler(agent, &mut result)
}

#[test]
fn test_parse_smoke() {
    let result = parse("Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 6.1; Trident/4.0)").unwrap();
    assert_eq!(result.name, "Internet Explorer".to_string());
}

#[test]
fn test_is_crawler_smoke() {
    assert!(!is_crawler("Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 6.1; Trident/4.0)"));
    assert!(is_crawler("Mozilla/5.0 (compatible; Yahoo! Slurp; \
                        http://help.yahoo.com/help/us/ysearch/slurp)"));
}
