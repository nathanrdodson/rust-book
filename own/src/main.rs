fn main() {
    let s1 = String::from("string");
	println!("{s1}");
	// perform move

	// "when you see a call to clone, you know that some arbitrary code is 
	// being executed and that code may be expensive. It’s a visual 
	// indicator that something different is going on."

	let s2 = s1.clone();
	// s1 now cloned
	println!("{s1}, {s2}");

	let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

	let mut t = String::from("world");
	let len = dont_own(&t);

	t += &String::from("test");

	println!("{t}");

    let x = 5;                      // x comes into scope

    makes_copy(x); // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn dont_own(s: &String) -> usize {
	s.len()
}

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.