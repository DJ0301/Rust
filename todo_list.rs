use std::io;

fn main() {
    let mut todo_list: Vec<String> = Vec::new();

    println!("Welcome to Rust Todo List!");

    loop {
        println!("\nPlease select an option:");
        println!("1. Add a todo");
        println!("2. View todos");
        println!("3. Mark todo as completed");
        println!("4. Exit");

        // Display the current todo list with indexes
        println!("Current Todo List:");
        if todo_list.is_empty() {
            println!("No todos found.");
        } else {
            for (index, todo) in todo_list.iter().enumerate() {
                println!("{}. {}", index + 1, todo);
            }
        }

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Enter todo:");
                let mut todo = String::new();
                io::stdin()
                    .read_line(&mut todo)
                    .expect("Failed to read line");
                todo_list.push(todo.trim().to_string());
                println!("Todo added successfully!");
            }
            2 => {
                println!("Todo List:");
                if todo_list.is_empty() {
                    println!("No todos found.");
                } else {
                    for (index, todo) in todo_list.iter().enumerate() {
                        println!("{}. {}", index + 1, todo);
                    }
                }
            }
            3 => {
                println!("Enter the index of todo to mark as completed:");
                let mut index_input = String::new();
                io::stdin()
                    .read_line(&mut index_input)
                    .expect("Failed to read line");
                let index: usize = match index_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input! Please enter a valid index.");
                        continue;
                    }
                };
                if index > 0 && index <= todo_list.len() {
                    println!("Marking todo '{}' as completed.", todo_list[index - 1]);
                    todo_list.remove(index - 1);
                } else {
                    println!("Invalid index! Please enter a valid index.");
                }
            }
            4 => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice! Please select a valid option.");
            }
        }
    }
}
