
#[derive(Debug)]
struct Person {
    name: String,
    fav_color: String,
    age: i32,
}

impl Person {
    fn new(name: String, fav_color: String, age: i32) -> Self {
        Self {
            name,
            fav_color,
            age,
        }
    }
}

fn print(data: &str) {
    println!("{:?}", data);
}

fn main() {
    let person: Person = Person::new(String::from("Abdul"), String::from("White"), 18);
    println!("{:?}", person);

    let people: Vec<Person> = vec![
        Person {
            name: String::from("John"),
            fav_color: String::from("Red"),
            age: 8,
        },
        Person {
            name: String::from("Doe"),
            fav_color: String::from("Black"),
            age: 9,
        },
        Person {
            name: String::from("Faith"),
            fav_color: String::from("Purple"),
            age: 14,
        },
    ];

    for person in people {
        if person.age <= 10 {
            print(&person.name);
            print(&person.fav_color);
        }
    }
}
