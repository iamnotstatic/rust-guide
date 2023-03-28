fn main() {
    let numbers: Vec<i32> = vec![10, 20, 30, 40];

    for num in &numbers {
        match num {
            30 => println!("Thirty"),
            _ => println!("{:?}", num),
        }
    }

    println!("List length is {:?}", numbers.len());
}
