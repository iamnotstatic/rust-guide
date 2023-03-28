fn main() {
    let number: i32 = 150;
    let is_gt_100: bool = number > 100;

    print_message(is_gt_100);
}

fn print_message(gt_100: bool) {
    match gt_100 {
        true => println!("it's big"),
        false => println!("it's small"),
    }
}
