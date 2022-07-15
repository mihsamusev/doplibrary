use std::collections::HashMap;
use serde::{Deserialize, Serialize};
pub struct Catalog {
    // catalog owns books?
    // index of books by ISBN { ISBN_1: Book1 }
    books_by_isbn: HashMap<String, Book>,
    // index of authors by ID { Author: Author }
    authors_by_id: HashMap<String, Author>
}

#[derive(Serialize, Deserialize)]
pub struct Book {
    title: String,
    publication_year: u32,
    isbn: String,
    author_ids: Vec<String>,
    book_items: Vec<BookItem> // data on physical copies of the book
}

// physical copies of the book
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct BookItem {
    id: String,
    library_id: String,
    // purchase_date: String,
    is_lent: bool
}
pub struct BookLending {
    date: String,
    book_item_id: String,
    isbn: String
}
pub struct Author {
    id: String,
    name: String,
    book_isbns: Vec<String>
}

pub struct BookItemInfo {}

pub fn search_book(catalog: &Catalog, search_query: &str) -> Option<Book> {
    todo!()
}

pub fn add_book_item(catalog: &Catalog, book_item_info: BookItemInfo) {
    todo!()
}

pub fn get_book_lendings(catalog: &Catalog, member_id: i32) -> Option<Vec<BookLending>> {
    todo!()
}

pub fn checkout_book(catalog: &Catalog, book_item_id: i32) {
    todo!()
}

pub fn return_book(catalog: &Catalog, book_item_id: i32) {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::catalog::*;
    #[test]
    fn deserialize_book() {
        let book_str = r#"{
            "isbn": "978-1779501127",
            "title": "Watchmen",
            "publication_year": 1987,
            "author_ids": ["alan-moore", "dave-gibbons"],
            "book_items": [
                {
                "id": "book-item-1",
                "library_id": "nyc-central-lib",
                "is_lent": true
                },
                {
                "id": "book-item-2",
                "library_id": "nyc-central-lib",
                "is_lent": false
                }
                ]
        }
        "#;
        let book: Book = serde_json::from_str(book_str).unwrap();
        assert_eq!(book.title, "Watchmen");
        assert_eq!(book.isbn, "978-1779501127");
        assert_eq!(book.publication_year, 1987);
        assert_eq!(book.author_ids, ["alan-moore", "dave-gibbons"]);
        assert_eq!(book.book_items[0], BookItem{id: "book-item-1".into(), library_id: "nyc-central-lib".into(), is_lent: true});
        assert_eq!(book.book_items[1], BookItem{id: "book-item-2".into(), library_id: "nyc-central-lib".into(), is_lent: false});
    }
}