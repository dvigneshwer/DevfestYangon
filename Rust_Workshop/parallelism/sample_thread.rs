//-- #########################
//-- Task: Spawning a thread in rust
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 19 March 17
//-- #########################

use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        "Hello from a thread!"
    });

    println!("{}", handle.join().unwrap());
}