struct GroceryItem {
    quantity: i32,
    id: i32,
}

fn display_quantity(item: &GroceryItem) {
    println!("Quantity {:?}", item.quantity);
}

fn display_id(item: &GroceryItem) {
    println!("Id {:?}", item.id);
}

fn main() {
    let my_time = GroceryItem {
        quantity: 50,
        id: 1,
    };

    display_quantity(&my_time);
    display_id(&my_time);
}
