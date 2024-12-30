Main resources:

- [»The book«](https://doc.rust-lang.org/stable/book/)

- To use `rust-analyzer` in LazyVim, you have to install it using the Rust toolchain, i.e. `rustup component add rust-analyzer`, instead of using `Mason` (<https://www.reddit.com/r/neovim/comments/1cpruok/rust_not_working_in_lazyvim/>).
- The installation of Rust also includes a local copy of the documentation, so that you can read it offline. Run `rustup doc` to open the local documentation in your browser.
- When adding a Rust dependency (aka _external crate_) to `Cargo.toml` under the `[dependencies]` section header, you get »instant feedback« about the version you are specifying.
- Running the `cargo doc --open` command will build documentation provided by all your dependencies locally and open it in your browser.
- In Rust, variables and references are immutable by default. To make them mutable, we use `mut`.

```rust
let apples = 5 // immutable
let mut bananas = 5 // mutable
```

- An _associated function_ is a function that's implemented on a type; for example in `String::new()`, the `::` syntax indicates that `new` is an associated function of the `String` type.
