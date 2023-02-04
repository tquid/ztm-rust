// Topic: Generics & Structures
//
// Requirements:
// * Create a Vehicle structure that is generic over traits Body and Color
// * Create structures for vehicle bodies and vehicle colors and implement the
//   Body and Color traits for these structures
// * Implement a 'new' function for Vehicle that allows it to have any body
//   and any color
// * Create at least two different vehicles in the main function and print their
//   info
//
// Notes:
// * Examples of car bodies can be Truck, Car, Scooter
// * Examples of colors could be red, white, black
// * It is not necessary to have data fields or function implementations
//   for the vehicle bodies/colors

trait Body {}
trait Color {}

struct Vehicle<T: Body, U: Color> {
    body: T,
    color: U,
}

impl<T: Body, U: Color> Vehicle<T, U> {
    fn new(body: T, color: U) -> Self {
        Self {
            body,
            color,
        }
    }
}

#[derive(Debug)]
struct Truck;
impl Body for Truck {}

#[derive(Debug)]
struct Bicycle;
impl Body for Bicycle {}

#[derive(Debug)]
struct Red;
impl Color for Red {}

#[derive(Debug)]
struct Blue;
impl Color for Blue {}

fn main() {
    let veras_bike = Vehicle::new(Bicycle, Red);
    let bobs_truck = Vehicle::new(Truck, Blue);
    println!("veras_bike is a {:?}-colored {:?}", veras_bike.color, veras_bike.body);
    println!("bobs_truck is a {:?}-colored {:?}", bobs_truck.color, bobs_truck.body);
}
