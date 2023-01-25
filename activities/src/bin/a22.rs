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
        return None;
    }
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{} {}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn check_clamp_equal() {
        assert_eq!(clamp(3, 3, 3), 3, "Equal inputs should make a sensible result");
    }
    #[test]
    fn check_clamp_lower() {
        assert_eq!(clamp(3, 4, 5), 4, "'n' lower than low bound should return low bound")
    }
    #[test]
    fn check_clamp_upper() {
        assert_eq!(clamp(6, 4, 5), 5, "'n' higher than high bound should return high bound")
    }
    #[test]
    fn check_clamp_negative() {
        assert_eq!(clamp(-100, -10, -1), -10, "Negative ranges should work")
    }
    #[test]
    fn check_div() {
        let result = div(10, 0);
        assert_eq!(result, None, "Divide by zero")
    }
}