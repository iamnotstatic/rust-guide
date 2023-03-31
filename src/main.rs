fn main() {
    use rust_guide::math;
    use rust_guide::msg;

    println!("{} {}", math::add(1, 2), msg::to_lowercase("HELLO WORLD"));
}
