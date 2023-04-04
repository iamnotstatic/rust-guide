fn longest<'a>(one: &'a str, two: &'a str) -> &'a str {
    if two > one {
        two
    } else {
        one
    }
}

fn main() {
    let short = "Hello";
    let long = "This is a long message";

    println!("{}", longest(short, long));
}
