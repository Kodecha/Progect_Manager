//main.rs by Brendan Kunderas
//Created on: June 12, 2023
//Last Edited on: June 12, 2023

//main.rs is the main file for the rust project

use std::io;

// Function for the todo list menu
fn todo_list() {
    println!("Welcome to the Todo List!");
    println!("Please select an option from the menu below:");
    println!("1. Add a Todo");
    println!("2. Remove a Todo");
    println!("3. View the Todo List");
    println!("4. Exit");
    println!("Please enter your selection: ");

    let mut selection = String::new();

    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line");

    println!("You selected: {}", selection);

    match selection.trim() {
        "1" => println!("You selected to add a Todo"),
        "2" => println!("You selected to remove a Todo"),
        "3" => println!("You selected to view the Todo List"),
        "4" => println!("You selected to exit the program"),
        _ => println!("Invalid selection, please try again"),
    }
}

// Function for the coding project file manager menu
fn coding_project_file_manager() {
    println!("Welcome to the Coding Project File Manager!");
    println!("Please select an option from the menu below:");
    println!("1. Add a Coding Project");
    println!("2. Remove a Coding Project");
    println!("3. View the Coding Project List");
    println!("4. Exit");
    println!("Please enter your selection: ");

    let mut selection = String::new();

    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line");

    println!("You selected: {}", selection);

    match selection.trim() {
        "1" => println!("You selected to add a Coding Project"),
        "2" => println!("You selected to remove a Coding Project"),
        "3" => println!("You selected to view the Coding Project List"),
        "4" => println!("You selected to exit the program"),
        _ => println!("Invalid selection, please try again"),
    }
}

// Function for creating a decent main menu in the cmd - customizable
fn display_main_menu(options: &[&str]) {
    let title = "Welcome to the Project Hub!";
    let prompt = "Please select an option:";
    let exit_option = "Exit";

    display_menu(title, prompt, options, exit_option);
}

// Similar to the main menu function, but for the todo list menu and other menus
fn display_menu(title: &str, prompt: &str, options: &[&str], exit_option: &str) {
    let title_width = title.len() + 4;
    let border = "═".repeat(title_width);

    println!("╔{}╗", border);
    println!("║ {} ║", title);
    println!("╠{}╣", border);
    println!("║ {} ║", prompt);
    println!("╠{}╣", border);

    for (index, option) in options.iter().enumerate() {
        println!("║ {}. {}", index + 1, option);
    }

    println!("╠{}╣", border);
    println!("║ 0. {} ║", exit_option);
    println!("╚{}╝", border);
}

// Function for the main menu
fn main_menu() {
    let options = ["Todo List", "Coding Project File Manager", "Other"];

    loop {
        display_main_menu(&options);

        let mut selection = String::new();

        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read line");

        let selection: usize = match selection.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match selection {
            1 => todo_list(),
            2 => coding_project_file_manager(),
            3 => println!("You selected Other"),
            0 => break,
            _ => println!("Invalid selection, please try again"),
        }
    }
}

fn main() {
    main_menu();
}