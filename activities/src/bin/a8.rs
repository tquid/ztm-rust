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

enum Flavour {
    Strawberry,
    Cola,
    Orange,
}

struct Serving {
    flavour: Flavour,
    ml: f64,
}

fn display_serving (serving: Serving) {
    let flavour = match serving.flavour {
        Flavour::Strawberry => "strawberry",
        Flavour::Cola => "cola",
        Flavour::Orange => "orange",
    };
    println!("This is a {:?} drink of {:?} mL", flavour, serving.ml);
}
fn main() {
    let small_coke = Serving {
        flavour: Flavour::Cola,
        ml: 150.0,
    };
    display_serving(small_coke);
}
