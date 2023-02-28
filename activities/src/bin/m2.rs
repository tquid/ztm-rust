// Topic: Writing implementations with macros
//
// Summary:
//   There are multiple id types for this program. Implement the `Deref` trait
//   for all id structures.
//
// Requirements:
//   * Create a macro which can generate an implementation block.
//   * Use the macro to implement Deref for:
//     * ContractorId
//     * EmployeeId
//     * GuestId
//     * InvestorId
//     * ManagerId
//     * VendorId
//
// Notes:
// * Use the existing `Deref` implementation as a template for your macro.
// * Run `cargo check --bin m2` to check your work. A successful check means
//   the activity is complete.

use std::ops::Deref;

#[derive(Debug)]
struct Id(usize);

macro_rules! generic_deref {
    ($type:ty) => {
        impl Deref for $type {
            type Target = Id;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

struct ContractorId(Id);
generic_deref!(ContractorId);
struct EmployeeId(Id);
generic_deref!(EmployeeId);
struct GuestId(Id);
generic_deref!(GuestId);
struct InvestorId(Id);
generic_deref!(InvestorId);
struct ManagerId(Id);
generic_deref!(ManagerId);
struct VendorId(Id);
generic_deref!(VendorId);

// impl Deref for ContractorId {
//     type Target = Id;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

/// This function can accept any type which can be dereferenced into an Id.
fn check_id(id: &Id) {
    println!("id {:?} ok!", id);
}

fn main() {
    // No need to edit the main function. These will be compiler errors until
    // there is an implementation of `Deref` for the Id types listed as part
    // of the program requirements.

    let contractor = ContractorId(Id(1));
    let employee = EmployeeId(Id(2));
    let guest = GuestId(Id(4));
    let investor = InvestorId(Id(3));
    let manager = ManagerId(Id(5));
    let vendor = VendorId(Id(6));

    check_id(&contractor);
    check_id(&employee);
    check_id(&guest);
    check_id(&investor);
    check_id(&manager);
    check_id(&vendor);
}
