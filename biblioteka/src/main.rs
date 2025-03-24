use std::fs::File;
use std::io::{self, Read};
use std::io;

#[derive(Debug)]
struct Book{
    title: String,
    content: String,
    id: u32, 
}


fn main(){
    // let mut file = File::open("poem.txt");
    // let mut contents = String::new();
    // file.expect("REASON").read_to_string(&mut contents);
    // println!("File Contents:\n{}", contents);
    
}