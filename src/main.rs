use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
struct Book {
    title: String,
    content: String,
    id: u32,
}

fn main() {
     // let mut file = File::open("poem.txt");
    // let mut contents = String::new();
    // file.expect("REASON").read_to_string(&mut contents);
    // println!("File Contents:\n{}", contents);


    let mut books: Vec<Book> = Vec::new();
    books.push(Book {
        title: "Poem".to_string(),
        content: "poem.txt".to_string(),
        id: 1,
    });

    loop {
        println!("\nCHOOSE AN OPTION:");
        println!("1 - READ A BOOK");
        println!("2 - ADD A BOOK");
        println!("3 - DELETE A BOOK");
        println!("4 - Quit\n");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Error reading input");
        let choice = choice.trim();

        match choice {
            "1" => {
                println!("Available books:");
                for book in &books {
                    println!("ID: {}, Title: {}", book.id, book.title);
                }
                println!("Enter the ID of the book to read: \n");
                
                let mut book_id = String::new();
                io::stdin().read_line(&mut book_id).expect("Error reading ID");
                let book_id: u32 = match book_id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a number.");
                        continue;
                    }
                };

                if let Some(book) = books.iter().find(|b| b.id == book_id) {
                    let mut file = match File::open(&book.content) {
                        Ok(f) => f,
                        Err(_) => {
                            println!("Could not open file: {}", book.content);
                            continue;
                        }
                    };
                    let mut contentsfile = String::new();
                    if file.read_to_string(&mut contentsfile).is_ok() {
                        println!("\n File Contents: \n{}", contentsfile);
                    } else {
                        println!("Could not read file contents.");
                    }
                } else {
                    println!("Book not found.");
                }
            }
            "2" => {
                println!("Enter book title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Error reading title");
                let title = title.trim().to_string();

                println!("Enter file name:");
                let mut content = String::new();
                io::stdin().read_line(&mut content).expect("Error reading file name");
                let content = content.trim().to_string();

                let new_id = books.len() as u32 + 1;
                books.push(Book {
                    title,
                    content,
                    id: new_id,
                });
                println!("Book added successfully!");
            }
            "3" => {
                println!("Enter the ID of the book to delete:");
                let mut book_id = String::new();
                io::stdin().read_line(&mut book_id).expect("Error reading ID");
                let book_id: u32 = match book_id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a number.");
                        continue;
                    }
                };

                if let Some(index) = books.iter().position(|b| b.id == book_id) {
                    books.remove(index);
                    println!("Book deleted successfully!");
                } else {
                    println!("Book not found.");
                }
            }
            "4" => {
                println!("Are you sure you want to quit? (y/n)");
                let mut quit_choice = String::new();
                io::stdin().read_line(&mut quit_choice).expect("Error reading choice");
                match quit_choice.trim() {
                    "y" => {
                        println!("Quitting...");
                        break;
                    }
                    "n" => println!("Okay, back to menu."),
                    _ => println!("Invalid option."),
                }
            }
            _ => {
                println!("Invalid option, please try again.");
            }
        }
    }
}
