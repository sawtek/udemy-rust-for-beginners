// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
    Cyan,
    Magenta, // Not used on purpose, to check out the compiler warning
    Yellow,
    Black,
}

fn print_color_name(color: Color) {
    let color_name = match color {
        Color::Cyan => "Cyan",
        Color::Magenta => "Magenta",
        Color::Yellow => "Yellow",
        Color::Black => "Black",
    };
    println!("{color_name}");
}

fn main() {
    print_color_name(Color::Black);
    print_color_name(Color::Yellow);
    print_color_name(Color::Cyan);
}
