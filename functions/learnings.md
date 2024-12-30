- In function signatures, you must declare the type of each parameter.
- You need to use single quotes to denote a value of type `char`, for example:
  ```Rust
  let greeting = "Hello, Sir."; // type `&str`
  let someChar = 'h'; // type `char`; note: compiler will throw `character literal may only contain one codepoint` if using more than one character
  ```
- Rust is an expression-based language:
  - **Statements** are instructions that perform some action and do not return a value.
  - **Expressions** evaluate to a resultant value.
    - Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.
    - Expressions can be part of statements: in `let y = 6;`, `6` is an expression.
    - Calling a function is an expression.
- A new scope block created with curly brackets is an expression:
  ```Rust
  fn main() {
    let y = {
  	  let x = 3;
  	  x + 1 // note the missing semicolon
    };

    println!("The value of y is: {y}"); // `y` will be 4
  }
  ```
- In Rust, the return value of a function is synonymous with the value of the final expression in the function's body. (However, you can still return early using the `return` keyword.)
- Return values are not named, however, their type is specified after an arrow `->`.
