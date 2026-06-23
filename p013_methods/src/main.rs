macro_rules! book_title {
    ($title:expr, $pages:expr) => {
        println!("book title: {}, count pages: {}", $title, $pages);
    };
}
struct Book {
    title: String,
    pages: u32,
}

impl Book {
    fn set_title(&mut self, new_title: String) {
        self.title = new_title;
    }

    fn reading_time(&self) -> u32 {
        self.pages
    }

    fn has_more_pages_than(&self, other: &Book) -> bool {
        self.pages > other.pages
    }

    fn default_book(pages: u32) -> Book {
        Book {
            title: String::from("Untitled"),
            pages,
        }
    }
}

fn main() {
    let mut book1 = Book {
        title: String::from("The Rust Programming Language"),
        pages: 500,
    };
    let book2 = Book {
        title: String::from("Learning Perl"),
        pages: 420,
    };
    let book3 = Book {
        title: String::from("Rust in Action"),
        pages: 520,
    };
    book_title!(book1.title, book1.pages);
    book_title!(book2.title, book2.pages);
    book_title!(book3.title, book3.pages);

    println!(
        "The reading time for '{}' is {} minutes.",
        book1.title,
        book1.reading_time()
    );
    println!(
        "Does '{}' have more pages than '{}'? {}",
        book1.title,
        book2.title,
        book1.has_more_pages_than(&book2)
    );
    println!(
        "Does '{}' have more pages than '{}'? {}",
        book1.title,
        book3.title,
        book1.has_more_pages_than(&book3)
    );
    let default_book: Book = Book::default_book(139);
    book_title!(default_book.title, default_book.pages);
    book1.set_title(String::from("New Title"));
    book_title!(book1.title, book1.pages);
}
