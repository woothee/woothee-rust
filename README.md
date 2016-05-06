# woothee-rust
The Rust implementation of Project Woothee, which is multi-language user-agent strings parsers. ( **Unofficial Woothee Project, NOW** )

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
