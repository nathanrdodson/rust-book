# Notes

I am completing this book: https://doc.rust-lang.org/book/

This will help me kickstart Rust stuff. Am I a Rustacean yet?

## Examples of data types that implement `Copy` trait:
- All the integer types, such as u32.
- The Boolean type, bool, with values true and false.
- All the floating-point types, such as f64.
- The character type, char.
- Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.