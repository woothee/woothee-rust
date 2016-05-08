# woothee-rust [![](https://travis-ci.org/hhatto/woothee-rust.svg?branch=master)](https://travis-ci.org/hhatto/woothee-rust)

The Rust implementation of [Project Woothee](https://github.com/woothee/woothee),
which is multi-language user-agent strings parsers. ( **Unofficial Woothee Project, NOW** )

## Usage

parsing user-agent.

```
extern crate woothee;

use woothee::parse;

fn main() {
    let result = parse("Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 6.1; Trident/4.0)");
    println!("{:?}", result);
}
```

run
```
Some(WootheeResult { name: "Internet Explorer", category: "pc", os: "Windows 7", os_version: "NT 6.1", browser_type: "UNKNOWN", version: "8.0", vendor: "Microsoft" })
```

## Benchmark
```
running 2 tests
test bench_uap     ... bench: 204,584,510 ns/iter (+/- 4,739,819)
test bench_woothee ... bench:     542,426 ns/iter (+/- 24,330)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured
```

## Links
* [on crates.io](https://crates.io/crates/woothee)
* [Documentation](https://hhatto.github.com/woothee-rust/woothee)
