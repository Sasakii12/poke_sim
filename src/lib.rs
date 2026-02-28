use std::fs::File;
use std::io:: ErrorKind;


pub fn team_file_create() {
    println!("Looking for teams.json....");
    File::open("teams.json").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            println!("teams.json not found, creating file..");
            File::create("teams.json").unwrap_or_else(|error| {
                panic!("Could not create file: {:?}", error);
            })
        } else {
            panic!("Error opening file: {:?}", error)
        }
    });
    println!("teams.json found!, loading teams...")
}

pub fn team_build_init() {

}