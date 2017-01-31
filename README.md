ASF Parser in Rust with nom 
===========================

[![Build Status](https://travis-ci.org/PyYoshi/asf.rs.svg?branch=master)](https://travis-ci.org/PyYoshi/asf.rs)

# Example

```rust
#[macro_use]
extern crate nom;
extern crate asf;

use nom::IResult;

fn main() {
    let input = include_bytes!("./assets/320x180_10fps.asf");
    let asf_obj = asf::parse_asf(input);

    match asf_obj {
        IResult::Done(_, v) => {
            println!("Done: {:?}", v);
        }
        IResult::Incomplete(a) => {
            panic!("Incomplete: {:?}", a);
        }
        IResult::Error(a) => {
            panic!("Error: {:?}", a);
        }
    }
}
```
