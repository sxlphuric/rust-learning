#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Level {
    High,
    Mid,
    Low,
    Zero
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct FoodItem {
        name: String,
        price: i32,
        description: String,
        emissions: Level,
}
    
impl FoodItem {
    pub fn new(name: &str, price: i32, description: &str, emissions: Level) -> Self {
        FoodItem {
            name: String::from(name),
            price,
            description: String::from(description),
            emissions
        }
            
    }
}



fn main() {
    let restaurant_name = "McSandwich";
    
    let mut food_items = vec![
        FoodItem::new("Basic Hamburger", 1, "Your basic hamburger. Always tastes like cardboard!", Level::High),
        FoodItem::new("French Fries", 3, "Delicious fries guaranteed to keep you wanting more.", Level::Mid),
        FoodItem::new("Milkshake", 4, "A delicious and creamy milk-based drink!", Level::Low),
        FoodItem::new("Chicken Strips", 3, "Yummy chicken strips. Approved by chicken enjoyers!", Level::Mid),
        FoodItem::new("Chicken Sandwich", 1, "A basic sandwich with juicy pieces of chicken.", Level::Zero),
        FoodItem::new("Fried Chicken", 6, "Delicious chicken made with breadcrumbs. Our best item!", Level::Mid),
    ];
    
    food_items.sort();

    println!("Welcome to {}! Here is our menu.", restaurant_name);
    println!("\n---------------------------------------\n");
    for i in food_items {
        println!(
            "| {} - {}$",
            i.name,
            i.price
        );
        let emissions = match i.emissions {
            Level::Zero => "None",
            Level::Low => "Low",
            Level::Mid => "Medium",
            Level::High => "High"
        };
        
        
        println!("| CO2 Emission level: {}", emissions);
        println!("|");
        println!("| {}\n", i.description);
    }
}
