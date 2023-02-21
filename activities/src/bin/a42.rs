// Topic: Implementing Iterator
//
// Summary:
// A game uses a scoring system that includes a score multiplier.
// The multiplier starts at 1 and increases by 1 each iteration.
// The amount the multiplier increases each iteration can be
// adjusted through in-game powerups.
//
// Example multiplier progression:
// 1, 2, 3, (+1 powerup obtained), 5, 7, 9, ...
//
// Requirements:
// * Write a program that uses an iterator to generate a score multiplier
// * The iterator must start at 1 and increase by 1 each iteration
//   * It must be possible to increase the per-iteration amount through powerups
//
// Notes:
// * Use the .next() method to advance the iterator to confirm it works correctly
// * Only the Iterator trait needs to be implemented for this activity

struct ScoreMultiplier {
    multiplier: usize,
    step: usize,
}

impl ScoreMultiplier {
    fn new() -> Self {
        Self {
            multiplier: 1,
            step: 1,
        }
    }
    fn powerup(&mut self, amount: usize) {
        self.step += amount;
    }
}

impl Iterator for ScoreMultiplier {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.multiplier += self.step;
        Some(self.multiplier)
    }
}

fn main() {
    let mut mult = ScoreMultiplier::new();
    println!("{:?}", mult.next());
    println!("{:?}", mult.next());
    println!("{:?}", mult.next());
    mult.powerup(1);
    println!("{:?}", mult.next());
    println!("{:?}", mult.next());
    mult.powerup(3);
    println!("{:?}", mult.next());
    println!("{:?}", mult.next());
    println!("{:?}", mult.next());
}
