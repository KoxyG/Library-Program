// A Library struct to manage a collection of books. Implement the following methods:

#[derive(Debug, PartialEq, Eq)]
struct Book {
    title: String,
    author: String,
    year: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct  Library {
    books: Vec<Book>,
}

impl Library {

    // A new empty library 
    fn new() -> Library {
        Library {
            books: Vec::new()
        }
    }

    // Add a book to the library
    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    // remove book from the library
    fn remove_book(&mut self, title: &str) -> Result<(), String> {   
       match self.books.iter().position(|b| b.title == title) {
              Some(index) => {
                self.books.remove(index);
                Ok(())
              },
              None => Err(format!("Book with title {} not found", title))
       }
    }

    // find books in the library
    fn find_books(&self, title: &str) -> Result<&Book, String> {
       match self.books.iter().find(|b| b.title == title) {
        Some(book) => Ok(book),
        None => Err(format!("Book with title '{}' not found", title)),
       }
    }

    // list books in the library
    fn list_books(&self) {   
        for book in &self.books {
            println!("Title: {}, Author: {}, Year: {}", book.title, book.author, book.year);
        }
    }

    // count books in the library
    fn count_books(&self) -> usize {
        self.books.len()
    }


}


fn main() {
    // Test the Library implementation
    let mut library = Library::new();

    // Create new books
    let book1 = Book {
        title: String::from("Rust book"),
        author: String::from("Progress Ochuko Eyaadah"),
        year: 1949,
    };

    let book2 = Book {
        title: String::from("Coding with stellar"),
        author: String::from("Koxy"),
        year: 1960,
    };

    // Add the books to the library
    library.add_book(book1);
    library.add_book(book2);

    // List all books in the library
    println!("All books in the library:");
    library.list_books();

    // Count the total number of books in the library
    println!("Total number of books: {}", library.count_books());

    // Find a book by title and print it if found
    match library.find_books("Rust Book") {
        Ok(book) => println!("Found book: Title: {}, Author: {}, Year: {}", book.title, book.author, book.year),
        Err(err) => println!("Error finding book: {}", err),
    }

    // Remove a book by title and print the result
    match library.remove_book("coding with stellar") {
        Ok(()) => println!("Removing book from library"),
        Err(err) => println!("Error removing book: {}", err),
    }

    // List all books in the library after removal
    println!("All books in the library after removal:");
    library.list_books();

    // Count the total number of books in the library after removal
    println!("Total number of books after removal: {}", library.count_books());
}
