// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Orange,
    Cherry,
}

struct Drink {
    flavor: Flavor,
    ounces: i32,
}

fn print_drink(drink: Drink) {
    println!(
        "{} flavor",
        match drink.flavor {
            // ok cool the language lets people do this...
            // notable that if i use a ?: here it prints the string with quotes surrounding it
            Flavor::Orange => "orange",
            Flavor::Cherry => "cherry",
        }
    );
    println!("{} fl oz", drink.ounces);
}

fn main() {
    let my_drink = Drink {
        flavor: Flavor::Cherry,
        ounces: 173,
    };
    print_drink(my_drink);

    print_drink(Drink {
        // ok cool the language lets people do this
        flavor: Flavor::Orange,
        ounces: 8,
    });
}
