use std::io;

fn main() {
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed to get console input");
    let x: u32 = x.trim().parse().expect("Failed to parse int");

    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("Failed to get console input");
    let y: u32 = y.trim().parse().expect("Failed to parse int");
    
    let z = x+y;

    println!("{}", z);
}