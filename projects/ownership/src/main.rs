// Example: Each value is Rust has a variable that's its owner.
// fn main() {
//     let s1 = String::from("RUST");
//     let len = calculate_length(&s1); // Send reference to s1
//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// Example: There can only be one owner at a time.
// fn main() {
//     let s1 = String::from("RUST");
//     let s2 = s1; // s1 is no longer valid

//     println!("{}", s2);
// }

// Example: When the owner goes out of scope, the value will be dropped
fn main() {
    let s1 = String::from("RUST");
    let len = calculate_length(&s1); // Send reference to s1
    println!("The length of '{}' is {}.", s1, len);
} // s1 goes out of scope here, and the value will be dropped

fn printLost(s: &String) {
    println!("{}", &s1); // s1 is no longer valid
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
