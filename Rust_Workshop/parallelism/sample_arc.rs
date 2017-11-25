//-- #########################
//-- Task: Understanding Arc
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 19 March 17
//-- #########################

use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

fn main() {
	let data = vec![1, 2, 3, 4];
	let idx = Arc::new(AtomicUsize::new(0));
	let other_idx = idx.clone();

	thread::spawn(move || {
	    other_idx.fetch_add(10, Ordering::SeqCst);
	});

	println!("{}", data[idx.load(Ordering::SeqCst)]);
}