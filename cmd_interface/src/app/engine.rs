use std::io;

use super::parser::*;
use super::models::User;
use super::controllers::AppController;
use super::views::*;
use super::enums::*;

use record_keeper::ErrorType;


// Engine class that runs the program.
pub struct Engine;
impl Engine {
    // Constructor
    pub fn new() -> Engine {
        Engine {}
    }

    // The program's main loop.
    pub fn run(& mut self) {

        let mut app_controller = AppController::from(
            Box::new(SplashPage), 
            User::new(),
            Box::new(SplashPageParser),
        );
        load_model(&mut app_controller);

        loop {
            // Display menu.
            app_controller.display();

            // Try to store user input in a String.
            let mut command_string = String::new();
            io::stdin().read_line(&mut command_string)
                .expect("Failed to read line.");

            // Trim input.
            command_string = command_string.trim().to_string();

            // Pass the command to the controller.
            match app_controller.process_command(command_string) {
                Some(s) => println!("{}", s),
                None    => break,
            }

            // End of loop.
        }

    }
}

fn load_model(controller: &mut AppController) {
    controller.add_client("Client 1".to_string());
    controller.add_task("Client 1".to_string(), "Task 1".to_string());
    controller.add_task("Client 1".to_string(), "Task 2".to_string());
    controller.add_client("Client 3".to_string());
    controller.add_client("Client 2".to_string());        
}
