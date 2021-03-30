# Sungod Ra
A simple and super slim random crate, gifted from the sun God!

If you need decent random numbers pretty speedily, and hate
to wait for compile-times, this is the crate for you!
No dependencies, no worries!

A basic usage would look like this:
```rust
use sungod::Ra;
fn main() {
    let mut ra = Ra::default();
    assert_ne!(ra.sample::<u64>(), ra.sample::<u64>());
}
```

This is an implementation of xorwow, in a nice slim package,
with some extra type safety. If you want to support randomizing
more exotic types, you'll have to implement it yourself. No
fancy traits or anything in this crate.

NOTE: This create is not at all suitable for cryptographic use.
