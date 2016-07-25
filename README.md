# woothee-rust [![](https://travis-ci.org/hhatto/woothee-rust.svg?branch=master)](https://travis-ci.org/hhatto/woothee-rust)

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
$ rustup run nightly cargo bench
running 3 tests
test bench_stabilizer ... bench:          14 ns/iter (+/- 1)
test bench_uap        ... bench: 210,071,986 ns/iter (+/- 27,302,537)
test bench_woothee    ... bench:      20,553 ns/iter (+/- 2,068)
```
[benchmark script](https://github.com/hhatto/woothee-rust/blob/master/tests/benchmark.rs)


## for Maintainer
generate code from woothee/woothee dataset & testsets.

```
$ cargo build --features=generate && cargo fmt
$ cargo test    # and code review!!!
```


## Links
* [on crates.io](https://crates.io/crates/woothee)
* [Documentation](https://hhatto.github.com/woothee-rust/woothee)
