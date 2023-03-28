struct Student {
    name: String,
    locker: Option<i32>,
}
fn main() {
    let mary: Student = Student {
        name: "Mary".to_owned(),
        locker: None,
    };

    match mary.locker {
        Some(num) => println!("Some locker {:?}", num),
        None => println!("None locker"),
    }
}
