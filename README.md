# woothee-rust [![](https://travis-ci.org/woothee/woothee-rust.svg?branch=master)](https://travis-ci.org/woothee/woothee-rust) [![Latest Version](https://img.shields.io/crates/v/woothee.svg)](https://crates.io/crates/woothee)

The Rust implementation of [Project Woothee](https://github.com/woothee/woothee),
which is multi-language user-agent strings parsers.


## Usage

parsing user-agent.

```rust
extern crate woothee;

use woothee::parser::Parser;

fn main() {
    let parser = Parser::new();
    let result = parser.parse("Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 6.1; Trident/4.0)");
    println!("{:?}", result);
}
```

run
```rust
Some(WootheeResult { name: "Internet Explorer", category: "pc", os: "Windows 7", os_version: "NT 6.1", browser_type: "UNKNOWN", version: "8.0", vendor: "Microsoft" })
```


## Benchmark
```
$ cargo +nightly bench
running 8 tests
test bench_stabilizer       ... bench:          27 ns/iter (+/- 2)
test create_parser_uap      ... bench: 233,401,600 ns/iter (+/- 8,151,668)
test create_parser_uaparser ... bench:  29,034,297 ns/iter (+/- 2,499,166)
test create_parser_woothee  ... bench:           1 ns/iter (+/- 0)
test parse_fast_uaparser    ... bench:     548,776 ns/iter (+/- 88,040)
test parse_uap              ... bench:     715,081 ns/iter (+/- 106,352)
test parse_uaparser         ... bench:   2,111,504 ns/iter (+/- 118,756)
test parse_woothee          ... bench:       8,741 ns/iter (+/- 605)
```
[benchmark script](https://github.com/woothee/woothee-rust/blob/master/benches/benchmark.rs)


## for Maintainer
generate code from woothee/woothee dataset & testsets.

```
$ cargo build --features=generate
$ cargo fmt
$ cargo test    # and code review!!!
```

with clippy lints (optional)

```
$ rustup component add clippy-preview && cargo clippy
```


## Links
* [on crates.io](https://crates.io/crates/woothee)
* [Documentation](https://woothee.github.com/woothee-rust/woothee)
