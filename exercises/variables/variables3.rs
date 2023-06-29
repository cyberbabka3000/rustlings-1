
use std::io;

fn main() {
    let mut x = String::new();

    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");
    println!("Number: {x}");
}
