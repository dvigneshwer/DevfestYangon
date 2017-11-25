// Task : To explain assignment operations in rust
// Author : Vigneshwer
// Version : 1.0
// Date : 3 Dec 2016

fn main(){
	// Arithmetic Operations
	println!("5 + 4 = {}", 5+4 );
	println!("5 - 4 = {}", 5-4 );
	println!("5 * 4 = {}", 5*4 );
	println!("5 / 4 = {}", 5/4 );
	println!("5 % 4 = {}", 5%4 );

	println!("********************");
	// Assigning data types and mathematical Operations
	let neg_4 = -4i32;
	println!("abs(-4) = {}", neg_4.abs() );
	println!("abs(-4) = {}", neg_4.pow(2) );
	println!("round(1.2345) = {}", 1.2354f64.round() );
	println!("ceil(1.2345) = {}", 1.2345f64.ceil() );
	print!("sin 3.14 = {}", 3.14f64.sin() );
}

// ref:
// https://doc.rust-lang.org/std/macro.println.html

// format!: write formatted text to String
// print!: same as format! but the text is printed to the console (io::stdout).
// println!: same as print! but a newline is appended.
// eprint!: same as format! but the text is printed to the standard error (io::stderr).
// eprintln!: sames as eprint!but a newline is appended.
