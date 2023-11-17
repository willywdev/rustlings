// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}

// Solution: Rust Shadowing:
// https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing
// You can initialize the variable again. This way the second variable overshadows the first.
// The first variable is still there, but you can't access it anymore.
