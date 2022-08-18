// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

use std::fmt;

enum Color {
    Blue,
    Red,
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Color::Blue => "blue",
                Color::Red => "red",
            }
        )
    }
}

struct Dimensions {
    height: f32,
    width: f32,
    depth: f32,
}

impl fmt::Debug for Dimensions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "h {} w {} d {}", self.height, self.width, self.depth)
    }
}

struct Box {
    dimensions: Dimensions,
    weight: f32,
    color: Color,
}

impl Box {
    fn new(dimensions: Dimensions, weight: f32, color: Color) -> Self {
        Self {
            dimensions: dimensions,
            weight: weight,
            color: color,
        }
    }

    fn print(&self) {
        println!(
            "dimensions: {:?} weight: {:?} color: {:?}",
            self.dimensions, self.weight, self.color
        );
    }
}

fn main() {
    let my_box = Box::new(
        Dimensions {
            height: 1.0,
            width: 2.0,
            depth: 3.0,
        },
        567.8,
        Color::Blue,
    );
    my_box.print();
    let other_dims = Dimensions {
        height: 4.0,
        width: 5.0,
        depth: 6.0,
    };
    let my_other_box = Box::new(other_dims, 678.9, Color::Red);
    my_other_box.print();
}
