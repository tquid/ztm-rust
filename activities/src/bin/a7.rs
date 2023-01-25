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

// * Use an enum with color names as variants
enum Color {
    Red,
    Blue,
    White,
    Black,
}

// * Use a function to print the color name
// * The function must use the enum as a parameter
fn color_str (color: Color) {
    // * Use a match expression to determine which color
    //   name to print
    match color {
        Color::Red => println!("red"),
        Color::Blue => println!("blue"),
        Color::White => println!("white"),
        Color::Black => println!("black"),
    }
}
fn main() {
    let color = Color::Black;
    color_str(color);
}
