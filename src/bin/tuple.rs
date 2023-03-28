fn coordinate() -> (i32, i32) {
    (1, 7)
}

fn main() {
    let (x, y) = coordinate();

    if y > 5 {
        println!(">5");
    } else if y < 5 {
        println!("<5")
    } else {
        println!("=5")
    }
}
