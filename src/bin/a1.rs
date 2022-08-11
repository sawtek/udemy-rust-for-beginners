// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn sub(a: i32, b: i32) -> i32 {
    a - b
}
fn main() {
    display_first_name();
    display_last_name();

    let division = 7 / 2;
    println!("Division {division}");
    let sub = sub(8, 3);
    println!("Sub {sub}");
}

fn display_first_name()
{
    println!("Alex");
}

fn display_last_name()
{
    println!("Wilson");
}