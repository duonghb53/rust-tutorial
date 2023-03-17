use std::{rc::Rc, sync::Arc};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
enum Role {
    Admin,
    Member,
    #[default]
    Guest,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct DB {}

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize)]
struct User {
    id: i32,
    name: String,
    role: Role,
    #[serde(skip)]
    db: Arc<DB>,
}

fn is_normal<T: Sync + Send>() {}

fn main() {
    let user = User {
        id: 1,
        name: String::from("DuongHB"),
        role: Role::Admin,
        db: Arc::new(DB {}),
    };
    println!("{:#?}", user);

    let user2 = user.clone();
    println!("{}", user == user2);

    let user = User::default();
    println!("{:#?}", user);
    is_normal::<User>();

    let user_str = "{ \"id\": 1,  \"name\": \"DuongHB\", \"role\" : \"Member\" }";
    let user: User = serde_json::from_str(&user_str).unwrap();
    println!("{:#?}", user);
}
