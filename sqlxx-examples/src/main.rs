use async_trait::async_trait;
use sqlxx_macros::Model;

#[derive(Debug, Model)]
pub struct User {
    id: i64,
    name: String,
    changes: {
        id: i64,
        name: String,
    }
}

// pub struct Person {}

// impl Person {
//     pub name: String;
// }

trait Save {
    fn save(&mut self);
}

fn main() {
    println!("fields {:?}", User::fields());

    UserChangeTracker {};

    let mut u = User {
        id: 1,
        name: "Vincent".to_string(),
    };

    u.save();

    println!("user {:?}", u);

    // User::fields()
    println!("Hello, world!");
}
