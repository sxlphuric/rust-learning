fn main() {
    use std::collections::HashMap;
    let restaurant_name = "McSandwich";

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
        (
            "Basic Hamburger",
            1,
            "Your basic hamburger. Always tastes like cardboard!",
        ),
    );
    iteminfo.insert(
        "fries",
        (
            "French Fries",
            3,
            "Delicious fries guaranteed to keep you wanting more.",
        ),
    );
    iteminfo.insert(
        "milkshake",
        ("Milkshake", 4, "A delicious and creamy milk-based drink!"),
    );
    iteminfo.insert(
        "chickenstrips",
        (
            "Chicken Strips",
            3,
            "Yummy chicken strips. Approved by chicken enjoyers!",
        ),
    );
    iteminfo.insert(
        "chickensandwich",
        (
            "Chicken Sandwich",
            1,
            "A basic sandwich with juicy pieces of chicken.",
        ),
    );

    println!("Welcome to {}! Here is our menu.", restaurant_name);
    println!("\n---------------------------------------\n");
    for i in menu {
        println!(
            "{} - {}$",
            iteminfo.get(i).unwrap().0,
            iteminfo.get(i).unwrap().1
        );
        println!("{}\n", iteminfo.get(i).unwrap().2)
    }
}
