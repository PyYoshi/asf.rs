ASF Parser in Rust with nom 
===========================

[![Build Status](https://travis-ci.org/PyYoshi/asf.rs.svg?branch=master)](https://travis-ci.org/PyYoshi/asf.rs)
[![Build status](https://ci.appveyor.com/api/projects/status/mmyxffueqp585tx2?svg=true)](https://ci.appveyor.com/project/PyYoshi/asf-rs)

# Example

```rust
extern crate nom;
extern crate asf;

fn main() {
    let input = include_bytes!("../assets/320x180_10fps.asf");
    let asf_obj = asf::parse_asf(input);

    match asf_obj {
        Ok((_, v)) => {
            println!("Done: {:?}", v);
        }
        Err(nom::Err::Incomplete(a)) => {
            panic!("Incomplete: {:?}", a);
        }
        Err(nom::Err::Error(a)) => {
            panic!("Error: {:?}", a);
        }
        Err(nom::Err::Failure(a)) => {
            panic!("Failure: {:?}", a);
        }
    }
}
```
