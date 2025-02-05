// -I- Immutable Reference
// fn main() {
//     let _x: i32 = 5;
//     let _r: &i32 = &_x;

//     println!("_x: {}", _x);
//     println!("_r: {}", _r);
// }

// -II- Mutable Reference
// fn main() {
//     let mut _x: i32 = 5; // mutable variable
//     let _r: &mut i32 = &mut _x; // mutable reference
//     *_r += 1;
//     *_r -= 4;

//     println!("_x: {}", _x);
// }


// Demonstration on one mutable reference or many immutable references

fn main() {
    let mut account = BankAccount {
        owner: "Alice".to_string(),
        balance: 100.0,
    };

    // Immutable borrow to check balance
    account.check_balance();

    // Mutable borrow to withdraw money
    account.withdraw(40.9);

    // Immutable borrow to check balance
    account.check_balance();
}
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing ${} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!("Account owned by {} has a balance of ${}", self.owner, self.balance);
    }
}
