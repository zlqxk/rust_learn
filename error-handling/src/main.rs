use std::{fs::File, io::ErrorKind};

fn main() {
    // let content = File::open("hello.txt");

    // let content = match content {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };

    let content = File::open("hello.txt").unwrap();
}
