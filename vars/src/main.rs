fn main() {
    let mut x = 5;
	println!("The value of x is: {}", x);
	x = 9;
	println!("The value of x is: {}", x);

	let y: u8 = 255;
	let z: i64 = -774234123;
	let f: f64 = 2.11;
	let mut d: f32 = 0.1;
	d = 0.111;

	let mut ops = 5.0 + 5.0;
	ops += 95.5 - 12.22;
	ops += 4.0 * 30.0;
	ops += 56.7 / 32.2;
	ops += -5.0 / 3.0; // -5/3 = -1.66~ which rounds down (i.e., "towards zero") to -1.
	ops += 48.0 % 5.0;

	let u: bool = ops > 25.0;

	let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻'; // emoji!
	// IMPORTANT NOTE ABOUT char!: it is a Unicode Scalar Value, which is a very powerful designation
	let mut i = 21.12;
	let mut x = 22;
	let mut u = 23;
	// You can declare tuples of varied types
	let tuple = (27.1, 4, 6);
	// or then something like
	(i, x, u) = tuple;
	// this ^^ is called destructuring
	// or use object reference notation like this:

	tuple.0 == 0.0; // = false

	// ARRAYS
	// arrays in Rust have a fixed length.

	let a = [1, 2, 3, 4, 5];
	let b = [3; 5]; // = [3, 3, 3, 3, 3]
}

// constant declaration example -- 
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

/*
	The compiler is able to evaluate a limited set of operations at compile time, which lets us choose to write out this value in a way that’s easier to understand and verify, rather than setting this constant to the value 10,800. See the Rust Reference’s section on constant evaluation for more information on what operations can be used when declaring constants.
*/
