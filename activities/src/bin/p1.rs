// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use std::io;
use std::collections::HashMap;
use itertools::Itertools;


fn bills_to_vec(bills: &HashMap<String, Vec<f64>>) -> Vec<(String, f64)> {
    let mut bill_list: Vec<(String, f64)> = Vec::new();
    for name in bills.keys().sorted() {
        for item in 0..bills[name].len() {
            bill_list.push((name.to_owned(), bills[name][item]));
        }
    }
    bill_list
}

fn show_bills(bills: &mut HashMap<String, Vec<f64>>) {
    let sorted_bills = bills_to_vec(bills);
    for item in 0..sorted_bills.len() {
        let (name, amount) = &sorted_bills[item];
        println!("\t[{}] {}: {:.2}", item + 1, name, amount);
    }
}

fn get_input() -> String {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => return buffer.trim().to_owned(),
        Err(e) => panic!("Error trying to read input: {:?}", e),
    };
}

fn get_float() -> f64 {
    loop {
        let response = get_input();
        let maybe_float = response.parse::<f64>();
        match maybe_float {
            Ok(ok_float) => return ok_float,
            Err(e) => println!("Not a floating point number: {:?}.\nPlease retry:", e),
        }
    }
}

fn get_int() -> u8 {
    loop {
        let response = get_input();
        let maybe_int = response.parse::<u8>();
        match maybe_int {
            Ok(int) => return int,
            Err(_) => println!("Not a number! Please try again."),
        }
    }
}

fn are_you_sure() -> bool {
    println!("Are you sure? (y/n)");
    loop {
        let response = get_input();
        match response.as_ref() {
            "y" => return true,
            "n" => return false,
            _ => println!("Please respond with \"y\" or \"n\".")
        }
    }
}

fn add_bill(bills: &mut HashMap<String, Vec<f64>>) {
    println!("Enter name for bill:");
    let name = get_input();
    println!("Enter amount:");
    let amount = get_float();
    if let Some(amounts) = bills.get_mut(&name) {
        amounts.push(amount);
    } else {
        bills.insert(name, vec![amount]);
    }
}

fn index_bills(bills: &mut HashMap<String, Vec<f64>>) -> HashMap<String, usize> {
    let sorted_bills = bills_to_vec(bills);
    let mut counter: usize = 0;
    let mut first_index: HashMap<String, usize> = HashMap::new();
    let mut current_name = String::new();
    for (name, _) in sorted_bills.iter() {
        if name != &current_name {
            current_name = name.clone();
            first_index.insert(name.to_owned(), counter);
        }
        counter += 1;
    }
    first_index
}

fn delete_bill(bills: &mut HashMap<String, Vec<f64>>) {
    let sorted_bills = bills_to_vec(bills);
    show_bills(bills);
    let first_index = index_bills(bills);
    loop {
        println!("Enter the number of a bill to delete:");
        let index = (get_int() - 1) as usize;
        if let Some(entry) = sorted_bills.get(index) {
            let (name, amount) = entry;
            println!("This will delete {}'s bill for {}.", name, amount);
            if are_you_sure() {
                bills.get_mut(name).map(|val| val.remove(index - first_index[name]));
            } else {
                println!("OK, will not delete.");
            }
            break;
        } else {
            println!("Bill number {} is out of range! Try again.", index + 1)
        }
    }
}

fn edit_bill(bills: &mut HashMap<String, Vec<f64>>) {
    let sorted_bills = bills_to_vec(bills);
    show_bills(bills);
    let first_index = index_bills(bills);
    loop {
        println!("Enter the number of a bill to edit:");
        let index = (get_int() - 1) as usize;
        if let Some(entry) = sorted_bills.get(index) {
            let (name, amount) = entry;
            println!("{}: {}", name, amount);
            println!("Enter a new amount:");
            let new_amount = get_float();
            println!("This will update the bill to be name: {} and amount: {}", name, new_amount);
            if are_you_sure() {
                bills.get_mut(name).map(|val| val[index - first_index[name]] = new_amount);
            } else {
                println!("OK, aborting edit.");
            }
            break;
        }
    }
}

fn main() {
    let mut bills: HashMap<String, Vec<f64>> = HashMap::new();
    loop {
        println!("Please enter an option:");
        println!("1. Add new bill");
        println!("2. List bills");
        println!("3. Edit a bill");
        println!("4. Delete a bill");
        println!("5. Quit");
        println!("----------------");
        let option = get_input();
        match option.as_ref() {
            "1" => add_bill(&mut bills),
            "2" => _ = show_bills(&mut bills),
            "3" => edit_bill(&mut bills),
            "4" => delete_bill(&mut bills),
            "5" => break,
            _ => println!("Invalid selection. Please try again."),
        }
    }
}
