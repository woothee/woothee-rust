# woothee-rust [![Rust](https://github.com/woothee/woothee-rust/workflows/Rust/badge.svg)](https://github.com/woothee/woothee-rust/actions) [![Latest Version](https://img.shields.io/crates/v/woothee.svg)](https://crates.io/crates/woothee)

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
running 5 tests
test bench_stabilizer       ... bench:           0.25 ns/iter (+/- 0.01)
test create_parser_uaparser ... bench: 193,413,350.00 ns/iter (+/- 2,899,137.53)
test create_parser_woothee  ... bench:           0.25 ns/iter (+/- 0.02)
test parse_uaparser         ... bench:     276,041.65 ns/iter (+/- 6,581.72)
test parse_woothee          ... bench:       1,490.25 ns/iter (+/- 32.03)
```
[benchmark script](https://github.com/woothee/woothee-rust/blob/master/benches/benchmark.rs)


## for Maintainer
generate code from woothee/woothee dataset & testsets.

```
$ cargo build --features=generate
$ cargo +nightly fmt
$ cargo test    # and code review!!!
```

with clippy lints (optional)

```
$ rustup component add clippy-preview && cargo clippy
```


## Links
* [on crates.io](https://crates.io/crates/woothee)
* [Documentation](https://woothee.github.com/woothee-rust/woothee)
