
mod vault;

use crate::vault::*;

fn main() {
    let test_entry = Entry {
        service: String::from("fakesite.com"),
        username: String::from("gurt"),
        password: String::from("12345"),
    };

    println!("password: {}", test_entry.service);
}
