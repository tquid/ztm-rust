// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter
trait Perimeter {
    fn calculate(&self) -> f32;
}

struct Square {
    side: f32,
}

struct Triangle {
    a: f32,
    b: f32,
    c: f32,
}

impl Perimeter for Square {
    fn calculate(&self) -> f32 {
        self.side * 4.0
    }
}

impl Perimeter for Triangle {
    fn calculate(&self) -> f32 {
        self.a + self.b + self.c
    }
}

fn print_perimeter(shape: impl Perimeter) {
    println!("{:?}", shape.calculate());
}

fn main() {
    println!("Perimeter for triangle:");
    print_perimeter(Triangle { a: 3.4, b: 6.8, c: 7.0 });
    println!("Perimeter for square:");
    print_perimeter(Square { side: 4.5 });
}
