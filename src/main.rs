use std::{fs::{File, OpenOptions, remove_file}, io::{Read, Write}};

fn main() {
    // let mut file = File::create("src/hello.txt").expect("failed");
    // file.write_all("Hello World \n".as_bytes()).expect("failed")

    // let mut file = OpenOptions::new().append(true)
    //     .open("src/hello.txt")
    //     .expect("cannot opened file");
    // file.write_all("Adding content \n".as_bytes()).expect("failed")

    // let mut file = File::open("src/hello.txt").unwrap();
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).unwrap();
    // println!("{}",contents);

    remove_file("src/hello.txt").expect("failed");
}
