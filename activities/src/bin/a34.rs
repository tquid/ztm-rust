// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state

struct Luggage<State> {
    id: i32,
    state: State,
}

impl<State> Luggage<State> {
    fn transition<NextState: std::fmt::Debug>(self, state: NextState) -> Luggage<NextState> {
        println!("transitioning luggage id {} to state {:?}", self.id, state);
        Luggage {
            id: self.id,
            state: state,
        }
    }
}
//   * Check-in        (passenger gives luggage to airport)
#[derive(Debug)]
struct CheckIn;
impl Luggage<CheckIn> {
    fn new(id: i32) -> Self {
        Self {
            id,
            state: CheckIn,
        }
    }
    fn check_in(self) -> Luggage<OnLoading> {
        self.transition(OnLoading)
    }
}
//   * OnLoading       (luggage is loaded onto correct plane)

#[derive(Debug)]
struct OnLoading;
impl Luggage<OnLoading> {
    fn offload(self) -> Luggage<OffLoading> {
        self.transition(OffLoading)
    }
}
//   * Offloading      (luggage is taken off plane at destination)

#[derive(Debug)]
struct OffLoading;
impl Luggage<OffLoading> {
    fn to_terminal(self) -> Luggage<AwaitingPickup> {
        self.transition(AwaitingPickup)
    }
}
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)

#[derive(Debug)]
struct AwaitingPickup;
impl Luggage<AwaitingPickup> {
    fn pick_up(self) -> Luggage<EndCustody> {
        self.transition(EndCustody)
    }
}
//   * EndCustody      (luggage was picked up by passenger)

#[derive(Debug)]
struct EndCustody;

fn main() {
    let my_luggage = Luggage::new(4);
    let finished_luggage = my_luggage.check_in().offload().to_terminal().pick_up();
}
