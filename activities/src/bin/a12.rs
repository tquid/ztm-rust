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

// * Use a struct to encapsulate the box characteristics
struct ShippingBox {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}

struct Dimensions {
    height: f64,
    width: f64,
    depth: f64,
}

// * Use an enum for the box color
enum Color {
    Red,
    Green,
    Blue,
}

impl ShippingBox {
    // * Implement functionality on the box struct to create a new box
    fn new(dimensions: Dimensions, weight: f64, color: Color) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }
    // * Implement functionality on the box struct to print the characteristics
    fn show(&self) {
        let color_str = match self.color {
            Color::Red => "red",
            Color::Green => "green",
            Color::Blue => "blue",
        };
        println!("height: {:?}", self.dimensions.height);
        println!("width: {:?}", self.dimensions.width);
        println!("depth: {:?}", self.dimensions.depth);
        println!("weight: {:?}", self.weight);
        println!("color: {:?}", color_str);
    }
}


fn main() {
    let my_box = ShippingBox::new(Dimensions {
            height: 180.0,
            width: 90.0,
            depth: 60.0,
        }, 90.0, Color::Red);
    my_box.show();
}
