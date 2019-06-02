use std::io;

use super::parser::*;
use super::models::User;
use super::controllers::AppController;
use super::views::*;
use super::enums::*;

use record_keeper::ErrorType;


// Engine class that runs the program.
pub struct Engine {
    controller: AppController,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            controller: AppController::from(
                Box::new(SplashPage), 
                User::new(),
                Box::new(SplashPageParser)),
        }
    }

    pub fn run(& mut self) {

        self.load_model();

        loop {
            // Display menu.
            self.display();

            // Try to store user input in a String.
            let mut command_string = String::new();
            io::stdin().read_line(&mut command_string)
                .expect("Failed to read line.");

            // Trim input.
            command_string = command_string.trim().to_string();

            // Pass the command to the controller.
            match self.controller.process_command(command_string) {
                Some(s) => println!("{}", s),
                None    => break,
            }

            // End of loop.
        }

    }

    fn load_model(&mut self) {
        self.controller.add_client("Client 1".to_string());
        self.controller.add_task("Client 1".to_string(), "Task 1".to_string());
        self.controller.add_task("Client 1".to_string(), "Task 2".to_string());
        self.controller.add_client("Client 3".to_string());
        self.controller.add_client("Client 2".to_string());        
    }

    pub fn display(&self) {
        self.controller.display();
    }
}
