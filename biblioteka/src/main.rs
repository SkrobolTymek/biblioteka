use std::fs::File;
use std::io::{self, Read};

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
    let mut books: Vec<Book> = Vec::new();


    books.push(Book{title: poem, content: "poem.txt".to_string(), id: 1});

    loop{
    println!("CHOOSE AN OPTION: ");
    println!("1 - READ A BOOK");
    println!("2 - ADD A BOOK");
    println!("3 - DELETE A BOOK ");
    println!("4 - quit \n");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("There was an error in reading the choice");
    let choice = choice.trim();
    match choice{
        "1"=>{
            println!("Ksiazka {}", book.title);
            let mut file = File::open("{}", book.content);
            let mut contentsfile = String::new();
            file.expect("REASON").read_to_string(&mut contentsfile);
            println!("File Contents:\n{}", contentsfile);
        }
        "2"=>{

        }
        "3"=>{

        }
        "4"=>{
            println!("Are you sure? y/n");
            let mut quitchoice = String::new();
            io::stdin().read_line(&mut quitchoice).expect("There was an error while reading the quit choice");
            let quitchoice = quitchoice.trim();
            match quitchoice{
                "y"=>{
                    println!("QUITTING....");
                    break;
                }
                "n"=>{
                    println!("okey-dokey");
                }
                _=>{
                    println!("You chose a non existing option");
                }
            }
        }
        _=>{
            println!("You chose a wrong option");
        }
    }
}


}