// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
    age: i32,
}

impl Customer {
    fn can_make_restricted(self) -> Result<(), String> {
        if self.age < 21 {
            Err("not old enough".to_owned())
        } else {
            Ok(())
        }
    }
}

fn main() {
    let my_customer = Customer { age: 19 };
    match my_customer.can_make_restricted() {
        Err(msg) => println!("{:?}", msg),
        Ok(_) => (),
    }

    let other_customer = Customer { age: 21 };
    println!("{:?}", other_customer.can_make_restricted());
}
