use std::collections::HashMap;
use crate::catalog::BookLending;

pub struct UserManagement {
    librarians_by_email: HashMap<Email, Librarian>,
    members_by_email: HashMap<Email, Member>,
}

pub struct Email(String);

pub struct Member {
    email: String,
    pasword_hash: String,
    is_blocked: bool,
    book_lendings: Vec<BookLending>
}

pub struct Librarian {
    email: String,
    pasword_hash: String
}

pub struct LoginInfo {}

pub fn block_member(user_management: UserManagement, member_id: i32) {
    todo!()
}

pub fn unblock_member(user_management: UserManagement, member_id: i32) {
    todo!()
}

pub fn login_user(user_management: UserManagement, login_info: LoginInfo) {
    todo!()
}

pub fn is_librarian(user_management: UserManagement, user_id: i32) -> bool {
    todo!()
}