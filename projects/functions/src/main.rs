// Entry point
fn main() {
    hello_world();
    tell_height(180);
    human_id("Alice", 30, 180.5);

    let x: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty // or return price * qty;
    };

    println!("Product: {:?}", x);
    // add(5, 10);
    let y: i32 = add(5, 10);
    println!("Sum: {}", y);
    println!("Sum from function `add`: {}", add(5, 10));

    let weight: f32 = 70.5;
    let height: f32 = 1.80;
    let bmi = calculate_bmi(weight, height);
    println!("BMI: {:.2}", bmi);
}

fn hello_world() {
    println!("Hello, Rust🦀!");
}

fn tell_height(height: u32) {
    println!("Height: {} cm", height);
}

fn human_id(name: &str, age: u32, height: f32) {
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Height: {} cm", height);
}

// function returning value
fn add(x: i32, y: i32) -> i32 {
    x + y
}

// Expression and Statements
// Expression: Returns a value
// Statement: Instruction that performs some action and does not return a value

//  BMI = weight(kg) / height(m) ^ 2

fn calculate_bmi(weight_kg:f32, height_m:f32) -> f32 {
    weight_kg / (height_m * height_m)
}
