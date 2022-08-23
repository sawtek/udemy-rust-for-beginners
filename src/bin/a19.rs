// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
enum ItemType {
    Chair,
    Bed,
    Table,
    Couch,
}

fn main() {
    let mut items: HashMap<ItemType, i32> = HashMap::new();
    items.insert(ItemType::Chair, 5);
    items.insert(ItemType::Bed, 3);
    items.insert(ItemType::Table, 2);
    items.insert(ItemType::Couch, 0);

    let mut sum: i32 = 0;

    for (item_type, item_count) in items.iter() {
        let msg = match item_count {
            0 => "out of stock".to_owned(),
            _ => format!("{:?}", item_count),
        };
        println!("{:?} {:?}", item_type, msg);
        sum += item_count;
    }
    println!("{:?} items in stock", sum);
}
