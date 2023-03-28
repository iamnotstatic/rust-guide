enum Color {
    Red,
    Green,
    Yellow,
}

fn print_color(color: Color) {
    match color {
        Color::Green => println!("Green"),
        Color::Red => println!("Red"),
        Color::Yellow => println!("Yello"),
    }
}

fn main() {
    print_color(Color::Red)
}
