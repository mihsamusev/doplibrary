use crate::users::{self, UserManagement, LoginInfo};
use crate::catalog::{self, Catalog, Book, BookItem, BookItemInfo, BookLending};

pub struct LibraryData {
    pub catalog: Catalog,
    pub user_management: UserManagement
}


pub fn search_book(library_data: LibraryData, search_query: &str) -> Option<Book> {
    todo!()
}

pub fn add_book_item(library_data: LibraryData, book_item_info: BookItemInfo) {
    todo!()
}

pub fn get_book_lendings(library_data: LibraryData, user_id: i32, member_id: i32) -> Option<Vec<BookLending>> {
    if users::is_librarian(library_data.user_management, user_id) {
        catalog::get_book_lendings(&library_data.catalog, member_id)
    } else {
        None
    }
}

pub fn checkout_book(library_data: LibraryData, user_id: i32, book_item_id: i32) {
    todo!()
}

pub fn return_book(library_data: LibraryData, user_id: i32, book_item_id: i32) {
    todo!()
}

pub fn block_member(library_data: LibraryData, member_id: i32) {
    todo!()
}

pub fn unblock_member(library_data: LibraryData, member_id: i32) {
    todo!()
}

pub fn login_user(library_data: LibraryData, login_info: LoginInfo) {
    todo!()
}
