fn main() {
    println!("Hello, world!");
	let mut temp = convert_temp(20.0, 'f');
	println!("{}", temp);

	// another_function(5);
	// loops();
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
	// loop {
	// 	println!("{}", 4 + 5);
	// }

	let mut count = 0;

	let result = loop {
		count += 1;

		if count == 10 {
			break count * 2;
		}
	};

	println!("Value after loop: {result}");

	// label loops for readability and granual control flow
	'count_up: loop {
		println!("Count = {count}");

		let mut remain = 11;

		loop {
			if remain == 9 {
				break;
			}

			if count == 2 {
				break 'count_up;
			}
			remain -= 1;
		}
		count -= 1;
	}

	// conditional loops with 'while'
	// act as expected; while a condition evaluates to true, the code runs; otherwise, it exits the loop.
	let mut num = 3;

	while num != 0 {
		println!("{num}...");
		num -= 1;
	}

	println!("We have liftoff!!!");

	// 'for' loops
	let a = [1, 2, 4, 8, 16];
	let mut idx = 0;

	while idx < 5 {
		println!("the value at index {idx} is {}", a[idx]);

		idx += 1;
	}

	// VS

	for e in a {
		println!("the value is: {}", e);
	}
}

fn convert_temp(t: f32, c: char) -> f32 {
	if (c == 'c') {
		(t - 32.0) * (5.0/9.0)
	} else if (c == 'f') {
		(t * (9.0/5.0)) + 32.0
	} else  {
		-0.0
	}
}