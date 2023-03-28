enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}

fn main() {
    let tickets: Vec<Ticket> = vec![
        Ticket::Backstage(50.00, "Billy".to_owned()),
        Ticket::Standard(15.00),
        Ticket::Vip(30.00, "Amy".to_owned()),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, holder) => {
                println!("Backstage ticket Holder: {:?} Price: {:?}", price, holder)
            }
            Ticket::Standard(price) => println!("Standard ticket Price: {:?}", price),
            Ticket::Vip(price, holder) => {
                println!("Vip ticket Holder: {:?} Price: {:?}", price, holder)
            }
        }
    }
}
