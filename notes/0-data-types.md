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

## Data Types

So Rust is a statically typed language, the compiler must know all the data types of variables in your code for the program to compile and run. You don't always have to explicitly type your variables. When many types are possible, you must inform the complier of the specific type using type annotations.

Rust comes with four built-in primitive data types. These are:

- Integer Numbers
- Floating Point Numbers
- Boolean
- Characters

if you don't specify the type for an integer, Rust automatically gives it a 23 bit signed integer.

Rust also have two floating points data types for decimals, which are the 32 bits and 64 bit.

```rust
fn main() {
    let nephew_age: u32 = 14;
    println!("My nephew is {}, years old", nephew_age);

    let float: f32 = 4.0; //   Float running on 32 bits 

    println!("1 + 2 is equal {}", 1+2)  ;          
    println!("1 x 2 is equal {}", 1*2)  ;          
}
```

Booleans in Rust is a data type used to store truths. Two possible value, eith true or false.

```rust
fn main() {
    ...

    let is_bigger_num = 2 > 4;
    println!("Is 2 > 4: {}", is_bigger_num);

    let is_lesser_num = 2 < 4;
    println!("Is 2 < 4: {}", is_lesser_num);   
}
```

Rust supports text values with two basic string types and one character type. A character is a single item while a string is a series of characters.

Not every string will be recognized on runtime.`&str` is like a pointer to a mutable string.

```rust
fn main() {
    let my_name: &str = "Anu"; // Pointer to a mutable string
}
```

Characters can be used and denoted by a single quotation mark, while strings can be used and denoted by double quotation marks.

```rust
fn main() {
    let my_name: &str = "Anu";
    let first_char: char = 'A';
    let last_char: char = 'u';
    let second_char: char = 'n';

    println!(
        "{} is the first character, {} is the last character, {} is the second character of my name {}",
        first_char,
        last_char,
        second_char,
        my_name
    );
}
```

```bash
A is the first character, u is the last character, n is the second character of my name Anu
```

### Data Collections

Tuples - a tuple is a grouping of different types collected into one compound value and the individual values are known as elements of the tuple. After a tuple is defined it can not grow or shrink, they have a fixed length.

The elements of the tuple can be accessed by an index position which starts at zero.

```rust
fn main() {
    let my_dog = ("Toby", 15, false);
    println!("My dog's name is {}, age is {}, and is a puppy: {}", my_dog.0, my_dog.1, my_dog.2);
}

```

```bash
My dog's name is Toby, age is 15, and is a puppy: false
```

Structs - a struct is a type composed of other types and elements in a structs are called fields. You can name each field to know what they mean.

Rust supports three struct types:

- Classic Structs
- Tuples Structs
- Units Structs

```rust
struct Student {
    name: String,
    level: u8,
    remote: bool,
}

fn main() {
    let student_1 = Student {
        name: String::from("Anu"),
        level: 12,
        remote: false,
    };

    println!(
        "Student 1's name is {}, level is {}, and is remote: {}",
        student_1.name,
        student_1.level,
        student_1.remote
    );
}
```

We can use struct like a tuple.

```rust
struct Grades(char, char, char, f32);

fn main() {
    let grades = Grades('A', 'B', 'C', 90.5);

    println!(
        "Grades: {}, {}, {}, with a weighted average of {}",
        grades.0, grades.1, grades.2, grades.3
    );
}
```
