# Notes

## Functions

Functions - A block of code that performs a specific task or set of tasks
After creating (declaration) a function, you go ahead to call the function.

In Rust, every program must have a main() function, which is the first function to run in the Rust program.

`fn` is the keyword for a function and the you declare the function name and parameters if any. The function body (in the curly brackets) is the part of the code that does the task of the function.

```rust
fn main() {

}
```

Macros are predefined functions in Rust.

```rust
fn main() {
    println!("Hello World") // A marco function
}
```

We can use macro function in several instances. For example in the println!() function.

```rust
fn main() {
    // Processes the arguments in order
    println!("Hello, {}!", "Anu") //    Output: Hello, Anu!
}
```

```rust
fn main() {
    // Processes the arguments in order
    println!("Hello, {} {}!", "Anu", "Adebayo") //    Output: Hello, Anu Adebayo!
}
```

Some substitution took place, something like a template string.

## Variables

In Rust, we declare our variables with the `let` keyword.

You can bind variables in two ways, implicitly or explicitly.

```rust
fn main() {
    let age = 33;   //  Implicit
    let birth_year;

    birth_year = 2004;    //  Explicit
}
```

In Rust, variables binding are immutable by default, and when they are immutable, a binded value can not be changed.

To be able to mutate a variable, you can use the `mut` keyword in front of your variable name.

```rust
fn main() {
    let mut age = 33;
    let birth_year;

    birth_year = 1991;

    println!("I am {} years old", age);

    age = 25;

    println!("I am now {} years old", age);
    println!("I was born in {}", birth_year)
}
```

```bash
Hello, Anu Adebayo!
I am 33 years old
I am now 25 years old
I was born in 2004
```

Since the `age` variable is now mutable, you can reassign the value of the variable anywhere in the block scope.

In Rust you can do something called `variable shadowing`, that is you can declare a new variable using the name of a pre-existing variable, and the new declaration will create a new binding. The old binding still exists but can not be reused in the block scope.

```rust
fn main() {
    println!("Hello, {} {}!", "Anu", "Adebayo");

    let mut age = 33;
    let birth_year = 1991;

    println!("I am {} years old", age);

    let birth_year = birth_year - 1;    //variable shadowing

    age = 25;

    println!("I am now {} years old", age);
    println!("I was born in {}", birth_year)
}
```

```bash
Hello, Anu Adebayo!
I am 33 years old
I am now 25 years old
I was born in 1990
```
