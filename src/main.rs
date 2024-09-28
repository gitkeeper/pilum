// Import modules
mod task;

// External crates
use std::io;

// Internal modules
use task::TaskList;

fn main() {
    let mut task_list = TaskList::new();

    loop {
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Exit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");
        let choice: u32 = match choice.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("Enter the title of the new task:");
                let mut title = String::new();
                io::stdin()
                    .read_line(&mut title)
                    .expect("Failed to read line.");
                task_list.add_task(title.trim().to_string());
            }
            2 => {
                task_list.list_tasks();
            }
            3 => {
                println!("Enter the ID of the completed item:");
                let mut id = String::new();
                io::stdin()
                    .read_line(&mut id)
                    .expect("Failed to read line.");
                let id: u64 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                task_list.complete_task(id);
            }
            4 => {
                println!("Exiting the program.");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter a number between 1 and 4.");
            }
        }
    }
}
