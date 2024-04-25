use std::io;

// Define a struct to represent menu items
#[derive(Debug, Clone)] // Adding Clone trait
struct MenuItem {
    name: String,
    price: f64,
}

fn main() {
    // Define menu items
    let menu_items = vec![
        MenuItem {
            name: "Burger".to_string(),
            price: 5.99,
        },
        MenuItem {
            name: "Pizza".to_string(),
            price: 8.99,
        },
        MenuItem {
            name: "Pasta".to_string(),
            price: 7.49,
        },
        MenuItem {
            name: "Salad".to_string(),
            price: 4.99,
        },
        MenuItem {
            name: "Drink".to_string(),
            price: 1.99,
        },
    ];

    println!("Welcome to DJ's Restaurant!");

    let mut order: Vec
    <MenuItem> = Vec::new();
    let mut total_price = 0.0;

    loop {
        println!("\nPlease select an option:");
        println!("1. Add item to order");
        println!("2. View current order");
        println!("3. Finish order");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: usize = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("\nPlease select an item from the menu:");
                println!("Menu:");
                for (index, item) in menu_items.iter().enumerate() {
                    println!("{}. {} - ${:.2}", index + 1, item.name, item.price);
                }

                let mut item_choice = String::new();
                io::stdin()
                    .read_line(&mut item_choice)
                    .expect("Failed to read line");

                let item_choice: usize = match item_choice.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input! Please enter a number.");
                        continue;
                    }
                };

                if item_choice > 0 && item_choice <= menu_items.len() {
                    let selected_item = &menu_items[item_choice - 1];
                    order.push(selected_item.clone()); // Cloning selected_item
                    total_price += selected_item.price;
                    println!("{} added to your order.", selected_item.name);
                } else {
                    println!("Invalid choice! Please select a valid item.");
                }
            }
            2 => {
                println!("Your current order:");
                if order.is_empty() {
                    println!("No items in the order.");
                } else {
                    for (index, item) in order.iter().enumerate() {
                        println!("{}. {} - ${:.2}", index + 1, item.name, item.price);
                    }
                    println!("Total: ${:.2}", total_price);
                }
            }
            3 => {
                println!("Your final order:");
                for item in &order {
                    println!("{} - ${:.2}", item.name, item.price);
                }
                println!("Total: ${:.2}", total_price);
                break;
            }
            _ => println!("Invalid choice! Please select a valid option."),
        }
    }
}
