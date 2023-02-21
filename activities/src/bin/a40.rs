// Topic: Smart Pointers & RefCell
//
// Summary:
//   A vehicle rental company wants to access the rentals available
//   at storefront locations. Create a program that provides access
//   to storefront rentals from the corporate headquarters.
//
// Requirements:
// * Corporate must be able to access the rentals at a storefront
// * Storefronts must be able to rent out vehicles
// * Rentals have the following attributes:
//   - Type of vehicle
//   - Vehicle Identification Number (VIN)
//   - Vehicle status:
//     * Available, Unavailable, Maintenance, Rented
//
// Notes:
// * Use Rc and RefCell to create shared mutable data structures
// * Create at least two rentals and ensure that Corporate and StoreFront
//   can both access the rental information
// * Test your program by changing the vehicle status from both a storefront
//   and from corporate

use std::{rc::Rc, cell::RefCell};

#[derive(Debug)]
enum VehicleType {
    Truck,
    Van,
    Sedan,
    Compact,
}

#[derive(Debug)]
enum VehicleStatus {
    Available,
    Unavailable,
    Maintenance,
    Rented,
}

#[derive(Debug)]
struct Rental {
    vehicle_type: VehicleType,
    vin: u64,
    status: VehicleStatus,
}

type Rentals = Rc<RefCell<Vec<Rental>>>;

#[derive(Debug)]
struct Corporate(Rentals);

#[derive(Debug)]
struct StoreFront(Rentals);

fn main() {
    let rentals: Rentals = Rc::new(RefCell::new(vec![]));
    let f150 = Rental {
        vehicle_type: VehicleType::Truck,
        vin: 134134,
        status: VehicleStatus::Rented,
    };
    let honda = Rental {
        vehicle_type: VehicleType::Compact,
        vin: 435763,
        status: VehicleStatus::Maintenance,
    };
    let buick = Rental {
        vehicle_type: VehicleType::Sedan,
        vin: 357676,
        status: VehicleStatus::Available,
    };
    let head_office = StoreFront(rentals.clone());
    let albuquerque = StoreFront(rentals.clone());
    albuquerque.0.borrow_mut().push(honda);
    dbg!(&albuquerque);
    dbg!(&head_office);
    head_office.0.borrow_mut().push(f150);
    dbg!(&albuquerque);
    dbg!(&head_office);
}
