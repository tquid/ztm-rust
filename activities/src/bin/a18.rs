// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

#[derive(Debug)]
struct Adult {
    age: i32,
    name: String,
}

impl Adult {
    fn new(age: i32, name: &str) -> Result<Self, String> {
        if age < 21 {
            return Err(format!("Cannot create record for {:?} as they are {:?} years old, less than 21", name, age));
        }
        Ok(Self { age, name: name.to_string() })
    }
}
fn main() {
    let candidates = [
        (34, "Meister Eckhart"),
        (19, "Harvey Businessman"),
    ];
    for person in candidates {
        let new_adult = Adult::new(person.0, person.1);
        match new_adult {
            Ok(verified) => println!("{:?} is {:?} years old", verified.name, verified.age),
            Err(err_str) => println!("{:?}", err_str),
        }
    }
}
