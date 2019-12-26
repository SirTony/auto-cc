# auto-cc

A small utility to perform automatic cycle collections on [bacon_rajan_cc](https://github.com/fitzgen/bacon-rajan-cc) objects when necessary.

This library exposes a single free-standing function named `cc` (signature: `fn<T: Trace>( value: T ) -> Cc<T>`)
that contains some logic to check the number of rooted objects, and perform a collection when the number of roots
exceeds some threshhold allowing it to function more similarly to a garbage collector, but otherwise just calls `Cc::new()` and returns the resulting `Cc<T>`.

This is intended for scenarios where precisely controlling the shape of data structures is impractical or impossible,
and large numbers of cycles are likely to happen outside of the programmer's control such as values in a VM/interpreter for a scripting language implementation.

The logic for checking and collecting roots incurs some additional overhead, and is only contained in the `cc` function (calling `Cc::new()` as per usual will not trigger the check). `Cc::new()` should be preferred for short-lived code, or code where little to no cycles are likely to be created.

# Example

```rust
extern crate auto_cc;

// auto-cc re-exports bacon_rajan_cc types, traits, and functions.
pub use auto_cc::*;

fn main() {
    let x: Cc<u32> = cc( 0u32 );
}
```
