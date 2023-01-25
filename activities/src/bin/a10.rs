// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn print_size(n: i32) {
    let is_small = n <= 100;
    match is_small {
        true => println!("It's small"),
        false => println!("It's big"),
    };
}

fn main() {
    let n = 100;
    print_size(n);
}
