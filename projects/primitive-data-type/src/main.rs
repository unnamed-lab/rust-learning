fn main() {
    //  Integers
    let x: i32 = -42;
    let y: u64 = 100;

    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);
    // diff bet i32 (32 bits) and i64 (64 bits)
    // range:
    // i32 - 2147483647
    // i64 - 9223372036854775807

    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;

    println!("Maximum value of i32: {}", e);
    println!("Maximum value of i64: {}", i);

    //  ====================================

    //  Floats
    let pi: f64 = 3.14;
    println!("Value of PI: {}", pi);

    // =====================================

    //  Boolean
    let is_snowing: bool = true;
    println!("Is it snowong? {}", is_snowing);

    //  =====================================

    //  Character Type
    let letter: char = 'a';
    println!("First letter of the alpgabet: {}", letter);
}
