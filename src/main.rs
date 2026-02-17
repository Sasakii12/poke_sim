use std::io;

pub mod prompts;

fn input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)
    .expect("Error taking input");

    buf
}


fn main() {
    println!("Hello, world!");
    let n = input();
    println!("{}", n);
}

