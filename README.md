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
running 8 tests
test bench_stabilizer       ... bench:          22 ns/iter (+/- 1)
test create_parser_uap      ... bench: 190,983,237 ns/iter (+/- 18,726,783)
test create_parser_uaparser ... bench: 331,963,075 ns/iter (+/- 20,320,543)
test create_parser_woothee  ... bench:           0 ns/iter (+/- 0)
test parse_fast_uaparser    ... bench:     423,632 ns/iter (+/- 42,520)
test parse_uap              ... bench:     606,682 ns/iter (+/- 65,967)
test parse_uaparser         ... bench:     826,622 ns/iter (+/- 105,940)
test parse_woothee          ... bench:       6,814 ns/iter (+/- 1,924)
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
