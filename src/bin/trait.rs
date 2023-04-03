struct Package {
    weight: f64,
}

impl Package {
    fn new(weight: f64) -> Self {
        Self { weight }
    }
}

impl Default for Package {
    fn default() -> Self {
        Self { weight: 3.0 }
    }
}

trait Perimeter {
    fn calculate_perimeter(&self) -> i32;
}

struct Square {
    side: i32,
}

impl Square {
    fn new(side: i32) -> Self {
        Self { side }
    }
}

impl Perimeter for Square {
    fn calculate_perimeter(&self) -> i32 {
        self.side * 4
    }
}

struct Triangle {
    side_a: i32,
    side_b: i32,
    side_c: i32,
}

impl Triangle {
    fn new(side_a: i32, side_b: i32, side_c: i32) -> Self {
        Self {
            side_a,
            side_b,
            side_c,
        }
    }
}

impl Perimeter for Triangle {
    fn calculate_perimeter(&self) -> i32 {
        self.side_a + self.side_b + self.side_c
    }
}

fn print_perimeter(shape: impl Perimeter) {
    let perimeter = shape.calculate_perimeter();
    println!("Perimeter = {:?}", perimeter);
}

fn main() {
    let package = Package::default();
    println!("Default: {:?}", package.weight);

    let square = Square::new(5);
    let triangle = Triangle::new(2, 3, 4);

    print_perimeter(square);
    print_perimeter(triangle);
}
