use std::{io, process};

use record_keeper;

mod menu;

fn program_loop() {
    println!("Please input a command.");
    loop {
        // Try to store user input in a String.
        let mut command = String::new();
        io::stdin().read_line(&mut command)
            .expect("Failed to read line.");

        println!("Your command: {}", &command);

        // Pass the command to the menu.
        if !menu::parse_command(command) {
            break;
        }

        // End of loop.
        println!("Please input a new command.");
    }
}

fn main() {
    //record_keeper::print_hello();
    
    // Display greeting message / spalsh page.
    println!("Welcome to Will's Ledger Program.");
    
    // Run main loop.
    program_loop();

    // Say goodbye / shudown program.
    println!("Quitting program.");
    process::exit(0);
}
