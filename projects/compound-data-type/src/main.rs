fn main() {
    // Array
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number Array: {:?}", numbers);

    let fruits: [&str; 3] = ["Apple", "Orange", "Grapes"];
    println!("Fruits Array: {:?}", fruits);

    println!("First Fruit: {}", fruits[0]);
    println!("Second Fruit: {}", fruits[1]);
    println!("Third Fruit: {}", fruits[2]);

    ////////////////////////////////////////

    // Tuples
    let human: (String, i32, bool) = ("Alice".to_string(), 30, true);
    println!("Human Tuple: {:?}", human);

    let my_mix_tuple: (i32, f64, char, [i8; 3]) = (42, 3.14, 'a', [1, 2, 3]);
    println!("My Mix Tuple: {:?}", my_mix_tuple);

    ////////////////////////////////////////

    // Slices
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number Slices: {:?}", number_slices);

    let animals_slices: &[&str] = &["Dog", "Cat", "Cow"];
    println!("Animals Slices: {:?}", animals_slices);

    let books_slices: &[String] = &[
        "The Rust Programming Language".to_string(),
        "Programming Rust".to_string(),
        "Rust in Action".to_string(),
    ];
    println!("Books Slices: {:?}", books_slices);

    ////////////////////////////////////////

    // String Vs String Slices (&str)
    // Strings [growable, heap-allocated data structure, mutable, owned]

    let mut stone_cold: String = String::from("Stone Cold Steve Austin");
    println!("Stone Cold: {}", stone_cold);
    stone_cold.push_str(" Yeah!");
    println!("Stone Cold: {}", stone_cold);

    // B- &str (String Slice)
    // String Slices [fixed-size, immutable, borrowed]

    let string: String = "Hello, World! 😊".to_string();
    let slice: &str = &string[0..5];
    println!("String Slice: {}", slice);

    let my_str_slice: &str = "Hello, World!";
    println!("My String Slice: {}", my_str_slice);
}
