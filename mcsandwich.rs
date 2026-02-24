#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct FoodItem {
        name: String,
        price: i32,
        description: String,
    }
    
    impl FoodItem {
        pub fn new(name: &str, price: i32, description: &str) -> Self {
            FoodItem {
               name: String::from(name),
                price,
                description: String::from(description) 
            }
            
        }
    }


fn main() {
    let restaurant_name = "McSandwich";
    
    let mut food_items = vec![
        FoodItem::new("Basic Hamburger", 1, "Your basic hamburger. Always tastes like cardboard!"),
        FoodItem::new("French Fries", 3, "Delicious fries guaranteed to keep you wanting more."),
        FoodItem::new("Milkshake", 4, "A delicious and creamy milk-based drink!"),
        FoodItem::new("Chicken Strips", 3, "Yummy chicken strips. Approved by chicken enjoyers!"),
        FoodItem::new("Chicken Sandwich", 1, "A basic sandwich with juicy pieces of chicken."),
        FoodItem::new("Fried Chicken", 6, "Delicious chicken made with breadcrumbs. Our best item!"),
    ];
    
    food_items.sort();

    println!("Welcome to {}! Here is our menu.", restaurant_name);
    println!("It is sorted by price for you poor people.");
    println!("\n---------------------------------------\n");
    for i in food_items {
        println!(
            "| {} - {}$",
            i.name,
            i.price
        );
        println!("| {}\n", i.description)
    }
}
