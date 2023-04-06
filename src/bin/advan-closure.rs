fn math(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    op(a, b)
}

fn main() {
    let username: &str = "iamnotstatic.eth";

    let add = Box::new(move |a, b| {
        println!("Print name {}", username);

        a + b
    });
    let sub = Box::new(|a, b| a - b);
    let mul = Box::new(|a, b| a * b);

    println!("{}", math(2, 2, add));
    println!("{}", math(2, 2, sub));
    println!("{}", math(2, 2, mul));
}
