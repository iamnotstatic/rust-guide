trait Body {}

trait Color {}

#[derive(Debug)]
struct Car;
impl Body for Car {}

#[derive(Debug)]
struct Truck;
impl Body for Truck {}

#[derive(Debug)]
struct Red;
impl Color for Red {}

#[derive(Debug)]
struct Blue;
impl Color for Blue {}

#[derive(Debug)]
struct Vehicle<B: Body, C: Color> {
    body: B,
    color: C,
}

impl<B: Body, C: Color> Vehicle<B, C> {
    pub fn new(body: B, color: C) -> Self {
        Self { body, color }
    }
}

fn main() {
    let red_truck = Vehicle::new(Truck, Red);
    let blue_car = Vehicle::new(Car, Blue);

    println!("{:?}", red_truck.body);
    println!("{:?}", blue_car.color);
}
