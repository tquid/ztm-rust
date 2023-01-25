// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

// * Use a struct for a persons age, name, and favorite color
struct PersonalData {
    age: i32,
    // * The color and name should be stored as a String
    name: String,
    favorite_color: String,
}


// * The name and colors should be printed using a function
fn print_name_and_color(data: &PersonalData) {
    println!("name: {:?}, favourite colour: {:?}", data.name, data.favorite_color);
}
// * Use an if expression to determine which person's info should be printed

fn main() {
    // * Create and store at least 3 people in a vector
    let drones = vec![
        PersonalData {
            name: "bob".to_owned(),
            age: 10,
            favorite_color: "orange".to_owned(),
        },
        PersonalData {
            name: "mary".to_owned(),
            age: 6,
            favorite_color: "blue".to_owned(),
        },
        PersonalData {
            name: "joseph".to_owned(),
            age: 35,
            favorite_color: "houndstooth".to_owned(),
        }
    ];
    // * Iterate through the vector using a for..in loop
    for person in drones {
        if person.age <= 10 {
            print_name_and_color(&person);
        }
    }
}