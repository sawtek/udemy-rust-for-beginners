// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    color: String,
}

impl Person {
    fn print(&self) {
        println!("Name: {:?}", self.name);
        println!("Favorite color: {:?}", self.color);
    }
}

fn main() {
    let people = vec![
        Person {
            age: 123,
            name: "Margaret".to_owned(),
            color: "Blue".to_owned(),
        },
        Person {
            age: 456,
            name: "Mr Dr Prof Patrick".to_owned(),
            color: "Pink".to_owned(),
        },
        Person {
            age: 1,
            name: "One".to_owned(),
            color: "All".to_owned(),
        },
    ];
    for person in &people {
        if person.age <= 10 {
            person.print();
        }
    }
}
