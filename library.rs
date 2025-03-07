#[derive(Debug, Clone)]
struct Book {
    title: String,
    author: String,
    isbn: String,
    is_issued: bool,
}

impl Book {
    fn issue_book(mut book: Book) -> Book {
        println!(
            "Issuing book: {} by {} (ISBN: {})",
            book.title, book.author, book.isbn
        );
        book.is_issued = true;
        book
    }
}


fn main() {
    let book = Book {
        title: "Rust Programming".to_string(),
        author: "Steve Klabnik".to_string(),
        isbn: "123456789".to_string(),
        is_issued: false,
    };
    let issued_book = Book::issue_book(book);
    println!("Book issued status: {}", issued_book.is_issued);

}

