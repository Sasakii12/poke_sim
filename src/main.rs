use std::io;

pub mod prompts;
mod pokemon;

fn input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)
    .expect("Error taking input");

    buf
}


fn main() {
    loop {
        println!("What would you like to do?");
        println!("[1] Battle, [2] Teambuild, [3] Quit");
        let n = input();
        match n.as_str().trim() {
            "1" => panic!("Not implemented"),
            "2" => prompts::team_file_create(),
            "3" => break,
            _ => {
                println!("Please enter a number as your option");
                continue;}
        }
    }
}

