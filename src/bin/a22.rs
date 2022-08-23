// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn clamp_low() {
        let actual = clamp(1, 10, 20);
        let expected = 10;
        assert_eq!(actual, expected, "wrong low clamp");
    }

    #[test]
    fn clamp_hi() {
        let actual = clamp(100, 10, 20);
        let expected = 20;
        assert_eq!(actual, expected, "wrong hi clamp");
    }

    #[test]
    fn clamp_no() {
        let actual = clamp(15, 10, 20);
        let expected = 15;
        assert_eq!(actual, expected, "wrong no change clamp");
    }

    #[test]
    fn div_test() {
        let actual = div(10, 5);
        let expected = Some(2);
        assert_eq!(actual, expected, "wrong division");
    }

    #[test]
    fn div_zero() {
        let actual = div(10, 0);
        let expected = None;
        assert_eq!(actual, expected, "wrong division by zero");
    }

    #[test]
    fn concat_test() {
        let actual = concat("ab", "cd");
        let expected = "abcd".to_owned();
        assert_eq!(actual, expected, "wrong concatenation");
    }
}
