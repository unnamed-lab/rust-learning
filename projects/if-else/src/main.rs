fn main() {
    if 1 == 2 {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }

    let sunny_day = true;
    let take_jacket = if sunny_day { "Don't take jacket" } else { "Take a jacket" };
    println!("{}", take_jacket);

    let num = 100;
    let out_of_range: bool;

    if num < 0 {
        out_of_range = true;
    } else if num == 0 {
        out_of_range = true;
    } else if num == 50 {
        out_of_range = true;
    } else {
        out_of_range = false;
    }
    println!("Is {} out of range: {}", num, out_of_range);
}
