struct User {
    id: i32,
    name: String,
}

fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();

    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(2),
        "katie" => Some(3),
        _ => None,
    }
}

fn main() {
    let name = "sam";

    let user = find_user(name).map(|user_id| User {
        id: user_id,
        name: name.to_owned(),
    });

    match user {
        Some(user) => println!("My name is {:?} and ID {:?}", user.name, user.id),
        None => println!("User not found"),
    }
}
