// Topic: Multithreading
//
// Requirements:
// * Run the provided functions in threads
// * Retrieve the data from the threads to print the message
//   "Hello, threads!"
//
// Notes:
// * Use the join function to wait for threads to finish

fn msg_hello() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "Hello, "
}

fn msg_thread() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "threads"
}

fn msg_excited() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "!"
}

fn main() {
    let hello = std::thread::spawn(|| {
        msg_hello()
    });
    let threads = std::thread::spawn(|| {
        msg_thread()
    });
    let excited = std::thread::spawn(|| {
        msg_excited()
    });
    let mut msg = "".to_owned();
    for thr in vec![hello, threads, excited] {
        match thr.join() {
            Ok(s) => msg.push_str(s),
            Err(_) => println!("Oops"),
        }
    }

    println!("{}", msg);
}
