use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();

    stock.insert("Chair", 5);
    stock.insert("Bed", 3);
    stock.insert("Table", 2);
    stock.insert("Couch", 0);
    stock.insert("Monitors", 0);

    let mut total_stock = 0;

    for (item, qty) in stock.iter() {
        total_stock = total_stock + qty;

        let stock_count = if qty == &0 {
            "out of stock".to_owned()
        } else {
            format!("{:?}", qty)
        };

        println!("item={:?}, stock={:?}", item, stock_count);
    }

    println!("Total stock={:?}", total_stock);
}
