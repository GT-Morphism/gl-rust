- Struct literal to create an instance of a struct

```Rust
struct Deck {
 cards: Vec<String>;
}

fn main () {
 let deck = Deck { cards: vec![] };
}
```

- _Inherent implementations_ are all about adding functionality to a struct (methods and associated functions); called »inherent« since they use the same name as the struct
  - Method: operates on a specific instance of a struct; to define a method the first parameter of the function has to be `&self` or `&mut self`
  - Associated function: tied to the struct definition

```Rust
impl Deck {
 fn new() -> Self { // associated function
  // stuff
 }

 fn shuffle(&self) { // method
  // stuff
 }
}

fn main () {
 let deck = Deck::new();

 deck.shuffle();
}
```

- `fmt::Display` for `{}` print marker (must be manually implemented)
- `fmt::Debug` for `{:?}` print marker (can derive (automatically create) the implementation using `#[derive(Debug)]`)

- `use` is about »aliasing«, while `mod` is for actually »importing« (see also: <https://github.com/rust-lang/book/issues/460#issuecomment-281549407>)
