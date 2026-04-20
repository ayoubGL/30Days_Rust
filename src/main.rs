use std::env;

fn main() {
    // 1. Collect arguments from the terminal
    let args: Vec<String> = env::args().collect();

    // 2. Check if the user provided enough commands
    if args.len() < 2 {
        println!("Usage: guardian <command> [argument...]\n");
        return;
    }

    // 3. The brain matching on the command
    let command = &args[1];
    let argument = &args[2];

    match command.as_str() {
        "add" => {
            print!("Action: Adding a new entry...{}", argument);
        }

        "list" => {
            println!("Action: Listing all entries...");
        }

        "search" => {
            println!("Action: Searching for entries...");
        }
        "delete" => {
            println!("Action: Deleting entries...");
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}
