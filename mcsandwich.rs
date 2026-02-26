use std::io;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]

enum Level {

    Zero,

    Low,

    Mid,

    High,

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

            emissions,

        }

    }

}

fn main() {

    let restaurant_name = "McSandwich";

    let mut food_items = vec![

        FoodItem::new(

            "Basic Hamburger",

            1,

            "Your basic hamburger. Always tastes like cardboard!",

            Level::High,

        ),

        FoodItem::new(

            "French Fries",

            3,

            "Delicious fries guaranteed to keep you wanting more.",

            Level::Mid,

        ),

        FoodItem::new(

            "Milkshake",

            4,

            "A delicious and creamy milk-based drink!",

            Level::Low,

        ),

        FoodItem::new(

            "Chicken Strips",

            3,

            "Yummy chicken strips. Approved by chicken enjoyers!",

            Level::Mid,

        ),

        FoodItem::new(

            "Chicken Sandwich",

            1,

            "A basic sandwich with juicy pieces of chicken.",

            Level::Zero,

        ),

        FoodItem::new(

            "Fried Chicken",

            6,

            "Delicious chicken made with breadcrumbs. Our best item!",

            Level::Mid,

        ),

    ];

    fn sort_food_items(food_items: &mut Vec<FoodItem>){
      let mut sorting_input = String::new();

      println!("What do you want to sort by?\n==> 1) Name\n==> 2) Price\n==> 3) CO2 Emissions");

      io::stdin()
          .read_line(&mut sorting_input) // Read input into the `input` variable
          .expect("Failed to input");

      let sorting: i32 = sorting_input.trim().parse().expect("Input not an integer");

      match sorting {
        1 => food_items.sort_by(|a, b| b.name.cmp(&a.name)),
        2 => food_items.sort_by(|a, b| b.price.cmp(&a.price)),
        3 => food_items.sort_by(|a, b| b.emissions.cmp(&a.emissions)),
        _ => {println!("Input invalid, sorting by price."); food_items.sort_by(|a, b| b.price.cmp(&a.price))}
      }

      let mut ascending_input = String::new();

      println!("\nSort in ascending order? true/false");

      io::stdin()
        .read_line(&mut ascending_input) // Read input into the `input` variable
        .expect("Failed to read line");

      let ascending: bool = ascending_input.trim().parse().expect("Input not an integer");

      if ascending {
        food_items.reverse();
      }
    }

    fn print_menu(food_items: Vec<FoodItem>) {
      for i in food_items {

          println!("| {} - {}$", i.name, i.price);

          let emissions = match i.emissions {

              Level::Zero => "None",

              Level::Low => "Low",

              Level::Mid => "Medium",

              Level::High => "High",

          };

          println!("| CO2 Emission level: {}", emissions);

          println!("|");

          println!("| {}\n", i.description);

      }
    }

    fn separator(length: i32, char: char) {
      let mut i = 0;
      let mut separator_string = String::new();
      while i < length {
        separator_string.push(char);
        i += 1
      }
      println!("\n{}\n", separator_string);
    }

    println!("Welcome to {}!", restaurant_name);

    sort_food_items(&mut food_items);

    separator(20, '-');

    print_menu(food_items);

}
