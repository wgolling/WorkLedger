use std::{io, process};

use record_keeper;

mod menu;

fn program_loop() {
    // Initialize menu engine
    let mut me = menu::Engine::new();

    loop {
        // Display menu.
        me.prompt();

        // Try to store user input in a String.
        let mut command = String::new();
        io::stdin().read_line(&mut command)
            .expect("Failed to read line.");

        command = command.trim().to_string();

        println!("Your command: {}", &command);

        // Pass the command to the menu engine.
        match me.parse_command(command) {
            Some(s) => println!("{}", s),
            None    => break,
        }

        // End of loop.
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
