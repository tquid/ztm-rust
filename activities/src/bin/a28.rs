// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

struct Shoes {
    color: Color,
}

impl Shoes {
    pub fn new(color: Color) -> Self {
        Self {
            color
        }
    }
}

struct Shirt {
    color: Color,
}

impl Shirt {
    pub fn new(color: Color) -> Self {
        Self {
            color
        }
    }
}

struct Pants {
    color: Color,
}

impl Pants {
    pub fn new(color: Color) -> Self {
        Self {
            color
        }
    }
}

fn shoes(color: Color) { // OMG. Shoes.
    println!("These shoes are {:?}", Shoes::new(color).color)
}

fn pants(color: Color) { // OMG. Shoes.
    println!("These pants are {:?}", Pants::new(color).color)
}

fn shirt(color: Color) { // OMG. Shoes.
    println!("This shirt is {:?}", Shoes::new(color).color)
}

fn main() {
    shoes(Color::Black);
    pants(Color::Custom("houndstooth".to_owned()));
    shirt(Color::Brown);
}

/* Messed this up, misunderstanding the directive that each fn should take an 
*unique* type. Interesting seeing how my desire to avoid "unnecessary" duplication
actually blew my legs off this time. As the lesson points out, if you wanted to 
restrict some articles of clothing to certain colors, this would require re-factoring.
Also the current design makes it easy to store a color as e.g. "shirt_color" but then
put that variable into the "shoes" fn. */