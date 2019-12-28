# auto-cc

A small utility to perform automatic cycle collections on [bacon_rajan_cc](https://github.com/fitzgen/bacon-rajan-cc) objects when necessary.

This is intended for scenarios where precisely controlling the shape of data structures is impractical or impossible, and large numbers of cycles are likely to happen outside of the programmer's control such as values in a VM/interpreter for a scripting language implementation.

The logic for checking and collecting roots is only contained in the `cc` function (calling `Cc::new()` as per usual will not trigger the check).

`Cc::new()` should be preferred for short-lived code, or code where little to no cycles are likely to be created.

It is important to note that cycles are collected _before_ a new `Cc<T>` is allocated, so it is possible that a dead cycle could still remain after the untrusted code has finished execution, and a manual call to the `collect_cycles` function from the `bacon_rajan_cc` crate to perform final cleanup is recommended.

# Basic Example

```rust
extern crate auto_cc;

use auto_cc::{
    cc,
    
    bacon_rajan_cc::{
        Cc,

        collect_cycles, // for final cleanup, when necessary
    }
};

fn main() {
    let _x: Cc<u8> = cc( 0 );
}

```
