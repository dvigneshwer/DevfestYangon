//-- #########################
//-- Task: Crashing threads using panic
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 19 March 17
//-- #########################

use std::thread;
fn main() {

	let handle = thread::spawn(move || {
	    panic!("oops!");
	});

	let result = handle.join();

	assert!(result.is_err());	
}

fn print_vec(vec: &Vec<i32>) {
    // the `vec` parameter is borrowed for this scope

    for i in vec.iter() {
        println!("{}", i)
    }

    // now, the borrow ends
}

fn use_vec() {
    let vec = make_vec();  // take ownership of the vector
    print_vec(&vec);       // lend access to `print_vec`
    for i in vec.iter() {  // continue using `vec`
        println!("{}", i * 2)
    }
    // vec is destroyed here
}

