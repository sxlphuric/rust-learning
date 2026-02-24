fn main() {
    use std::collections::HashMap;
    let restaurant_name = "McSandwich";

    struct FoodItem {
        name: String,
        price: i32,
        description: String,
    }

    let menu = [
        "hamburger",
        "fries",
        "milkshake",
        "chickenstrips",
        "chickensandwich",
    ];
    let mut iteminfo = HashMap::new();
    iteminfo.insert(
        "hamburger",
        FoodItem {
            name: String::from("Basic Hamburger"),
            price: 1,
            description: "Your basic hamburger. Always tastes like cardboard!".to_string(),
        },
    );
    iteminfo.insert(
        "fries",
        FoodItem {
            name: String::from("French Fries"),
            price: 3,
            description: "Delicious fries guaranteed to keep you wanting more.".to_string(),
        },
    );
    iteminfo.insert(
        "milkshake",
        FoodItem {
            name: String::from("Milkshake"),
            price: 4,
            description: "A delicious and creamy milk-based drink!".to_string(),
        },
    );
    iteminfo.insert(
        "chickenstrips",
        FoodItem {
            name: String::from("Chicken Strips"),
            price: 3,
            description: "Yummy chicken strips. Approved by chicken enjoyers!".to_string(),
        },
    );
    iteminfo.insert(
        "chickensandwich",
        FoodItem {
            name: String::from("Chicken Sandwich"),
            price: 1,
            description: "A basic sandwich with juicy pieces of chicken.".to_string(),
        },
    );

    println!("Welcome to {}! Here is our menu.", restaurant_name);
    println!("\n---------------------------------------\n");
    for i in menu {
        println!(
            "| {} - {}$",
            iteminfo.get(i).unwrap().name,
            iteminfo.get(i).unwrap().price
        );
        println!("| {}\n", iteminfo.get(i).unwrap().description)
    }
}
