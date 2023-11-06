fn main() {
    println!("Hello, world!");
	another_function(5);
	loops();
}

fn another_function(x: i32) {
	if x > 5 {
		println!("Function: GREATER THAN 5!");
	} else {
		println!("Another function: {x}");
	}
	let number = if x < 5 { 123 } else { 3 };
}

fn loops() {
	loop {
		println!("{}", 4 + 5);
	}
}