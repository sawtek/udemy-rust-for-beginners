// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum TicketType {
    Backstage(String, i32),
    Vip(String, i32),
    Standard(i32),
}

fn main() {
    let tickets = vec![
        TicketType::Backstage(String::from("Mr Backstage"), 123),
        TicketType::Vip(String::from("Ms Vip"), 234),
        TicketType::Standard(10),
    ];
    for ticket in &tickets {
        match ticket {
            TicketType::Backstage(name, price) => {
                println!("backstage name {:?} ${:?}", name, price)
            }
            TicketType::Vip(name, price) => {
                println!("vip name {:?} ${:?}", name, price)
            }
            TicketType::Standard(price) => {
                println!("standard ${:?}", price)
            }
        }
    }
}
