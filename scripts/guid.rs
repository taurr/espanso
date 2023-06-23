#!cargo

//! ```cargo
//! [package]
//! edition = "2021"
//! [dependencies]
//! uuid = { version="1.3.4", features=["v4"] }
//! ```

use uuid::Uuid;

fn main() {
    let uuid = Uuid::new_v4();
    println!("{}", uuid);
}
