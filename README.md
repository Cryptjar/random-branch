Random Branch
=============

[![Crates.io](https://img.shields.io/crates/v/random-branch.svg)](https://crates.io/crates/random-branch)
[![Documentation](https://docs.rs/random-branch/badge.svg)](https://docs.rs/random-branch)


<!-- cargo-sync-readme start -->

Provides a macro to select a random branch.

This crate provides the [`branch`](https://docs.rs/random-branch/latest/random-branch/macro.branch.html) and
[`branch_using`](https://docs.rs/random-branch/latest/random-branch/macro.branch_using.html) macro, which will
execute randomly one of the given expressions.

It is maybe best visualized by the following example:

```rust
branch!(
    println!("First line."),
    println!("Second line?"),
    println!("Third line!"),
);
```

This will be turned into something similar to this:

```rust
match rand::thread_rng().gen_range(0..3) {
    0 => println!("First line."),
    1 => println!("Second line?"),
    2 => println!("Third line!"),
    _ => unreachable!(),
}
```

For more details see [`branch`](https://docs.rs/random-branch/latest/random-branch/macro.branch.html) and
[`branch_using`](https://docs.rs/random-branch/latest/random-branch/macro.branch_using.html). The basic difference between them is,
that `branch` uses [`rand::thread_rng()`](rand::thread_rng\(\)) whereas
`branch_using` uses the the given [`rand::Rng`](rand::Rng).


<!-- cargo-sync-readme end -->


## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.