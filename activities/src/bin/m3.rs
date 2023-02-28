// Topic: Basic macro repetitions
//
// Requirements:
//   * Create a macro to generate hashmaps.
//   * The macro must be able to accept multiple key/value pairs.
//   * Print out the generated hashmap using the `dbg!` macro to ensure it works.

macro_rules! hashmaker {
    (
        $($key:tt : $val:tt),+
        $(,)?
    ) => {{
        let mut h = ::std::collections::HashMap::new();
        $(
            h.insert($key, $val);
        )+
        h
    }};
}

fn main() {
    let foo = hashmaker!("bob": 1, "mary": 2);
    dbg!(foo);
}
