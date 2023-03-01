// Topic: Macro practice
//
// Summary:
//   Create a macro that can be used to generate new test cases for
//   the function provided.
//
// Requirements:
// * Write a macro to generate tests for `sample_fn`
// * Create at least 6 test cases using the macro
//   * Test the minimum and maximum values for each match arm
// * All test functions must be created by invoking the macro
//
// Notes:
// * Tuples can be used to specify both the input and expected output
// * The macro can be invoked multiple times; repetitions are optional

#[derive(Debug, PartialEq)]
enum Size {
    Small,
    Medium,
    Large,
}

fn sample_fn(n: u8) -> Size {
    use Size::*;
    match n {
        0..=53 => Small,
        54..=154 => Medium,
        155.. => Large,
    }
}

fn main() {
    // use `cargo test --bin m5` to check your work
}

#[cfg(test)]
mod test {
    use super::{sample_fn, Size::*};
    macro_rules! size_tester {
        ($x:expr, $fn_name:ident, $size:expr) => {
            #[test]
            fn $fn_name() {
                assert_eq!(sample_fn($x), $size);
            }
        };
    }

    size_tester!(0, small_lower, Small);
    size_tester!(53, small_upper, Small);
    size_tester!(54, medium_lower, Medium);
    size_tester!(154, medium_upper, Medium);
    size_tester!(155, large_lower, Large);
    size_tester!(255, large_upper, Large);
}
