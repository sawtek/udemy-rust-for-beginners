// Project 1: Interactive bill manager
//
// User stories:
// * L1: I want to add bills, including the name and amount owed.
// * L1: I want to view existing bills.
// * L2: I want to remove bills.
// * L3: I want to edit existing bills.
// * L3: I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at level 1, but a
//   hashmap will be easier to work with at levels 2 and 3.
// * Create a function just for retrieving user input, and reuse it
//   throughout your program.
// * Create your program starting at level 1. Once finished, advance to the
//   next level.

use std::io;

struct Bill {
    name: String,
    owed: f64,
}

impl Bill {
    fn new_via_prompts() -> Option<Bill> {
        let mut name = String::new();
        let mut amount = String::new();
        println!("Name:");
        if io::stdin().read_line(&mut name).is_err() {
            return None;
        }
        println!("Amount:");
        if io::stdin().read_line(&mut amount).is_err() {
            return None;
        }
        let amt_num = amount.trim().parse::<f64>();
        let name = name.trim().to_owned();
        match amt_num {
            Ok(amount) => Some(Bill::new(&name, amount)),
            Err(_) => None,
        }
    }
    fn new(name: &String, owed: f64) -> Bill {
        Bill {
            name: name.to_owned(),
            owed: owed,
        }
    }
}

fn get_idx_from_input() -> Result<usize, ()> {
    let mut buf = String::new();
    if io::stdin().read_line(&mut buf).is_err() {
        return Err(());
    }

    let idx = buf.trim().parse::<usize>();
    match idx {
        Err(_) => Err(()),
        Ok(idx) => {
            if idx < 1 {
                println!("Invalid index");
                Err(())
            } else {
                Ok(idx)
            }
        }
    }
}

fn list_bills(bills: &mut Vec<Bill>) -> Result<(), ()> {
    let mut idx = 0;
    for bill in bills {
        idx += 1;
        println!("{:?}. {:?}: {:?}", idx, bill.name, bill.owed);
    }
    Ok(())
}

fn remove_bills(bills: &mut Vec<Bill>) -> Result<(), ()> {
    list_bills(bills)?;
    println!("Which bill to remove?");

    let idx = get_idx_from_input()?;
    bills.remove(idx - 1);
    Ok(())
}

fn edit_bill(bills: &mut Vec<Bill>) -> Result<(), ()> {
    list_bills(bills)?;
    println!("Which bill to edit?");

    let idx = get_idx_from_input()?;

    match Bill::new_via_prompts() {
        Some(new_bill) => {
            bills[idx - 1] = new_bill;
            Ok(())
        }
        None => Err(()),
    }
}

/**
 * Return Err on quit
 * Return Ok otherwise
 */
fn handle_option(data: &str, bills: &mut Vec<Bill>) -> Result<(), ()> {
    let x = match data {
        "1" => match Bill::new_via_prompts() {
            Some(bill) => {
                bills.push(bill);
                Ok(())
            }
            None => Err(()),
        },
        "2" => list_bills(bills),
        "3" => remove_bills(bills),
        "4" => edit_bill(bills),
        "q" => return Err(()),
        _ => return Ok(()),
    };
    match x {
        Ok(_) => Ok(()),
        _ => {
            println!("error");
            Ok(())
        }
    }
}

fn main() {
    let mut bills = Vec::new();
    loop {
        println!("What do you want to do?");
        println!("1) Add a new bill");
        println!("2) List bills");
        println!("3) Remove bill");
        println!("4) Edit bill");
        println!("q) quit");

        let mut buffer = String::new();
        let status = io::stdin().read_line(&mut buffer);
        match status {
            Ok(_) => match handle_option(&buffer.trim().to_owned(), &mut bills) {
                Ok(_) => (),
                Err(_) => break,
            },
            Err(_) => println!("error getting input"),
        }
    }
}
