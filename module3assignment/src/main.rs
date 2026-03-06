use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro
    let mut file = File::create(filename).unwrap();
    //write books to file
    for book in books{
       writeln!(file, "{},{},{}", book.title, book.author, book.year).unwrap();
    }

}

fn load_books(filename: &str) -> Vec<Book> {
    // TODO: Implement this function
    // Hint: Use File::open() and BufReader
    let file = File::open(filename).unwrap();
    let read = BufReader::new(file);

    let mut books: Vec<Book> = Vec::new();

    for line in read.lines(){
        let line = line.unwrap();
        let info: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        let book = Book{
            title: info[0].to_string(),
            author: info[1].to_string(),
            year: info[2].parse().unwrap(),//parsing to turn it back to u16 type
        };
        books.push(book);
    }
    books


}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}