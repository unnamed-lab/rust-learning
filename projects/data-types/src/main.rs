struct Student {
    name: String,
    level: u8,
    remote: bool,
}

struct Grades(char, char, char, f32);
fn main() {
    println!("Hello, {} {}!", "Anu", "Adebayo");

    let mut age = 33;
    let birth_year = 1991;

    println!("I am {} years old", age);

    let birth_year = birth_year - 1;

    age = 25;

    println!("I am now {} years old", age);
    println!("I was born in {}", birth_year);

    let nephew_age: u32 = 14;
    println!("My nephew is {}, years old", nephew_age);

    let float: f32 = 4.0; //   Float running on 32 bits

    println!("1 + 2 is equal {}", 1 + 2);
    println!("1 x 2 is equal {}", 1 * 2);

    let is_bigger_num = 2 > 4;
    println!("Is 2 > 4: {}", is_bigger_num);

    let is_lesser_num = 2 < 4;
    println!("Is 2 < 4: {}", is_lesser_num);

    let my_name: &str = "Anu"; // Pointer to a mutable string
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

    let my_dog = ("Toby", 15, false);
    println!("My dog's name is {}, age is {}, and is a puppy: {}", my_dog.0, my_dog.1, my_dog.2);

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

    let grades = Grades('A', 'B', 'C', 90.5);

    println!(
        "Grades: {}, {}, {}, with a weighted average of {}",
        grades.0, grades.1, grades.2, grades.3
    );
}
