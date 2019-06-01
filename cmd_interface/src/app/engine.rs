use std::io;

use super::parser::*;
use super::models::User;
use super::controllers::AppController;
use super::views::*;
use super::enums::*;

use record_keeper::{ErrorType, NotFoundError};


// Engine class that runs the program.
pub struct Engine {
    controller: AppController,
    parser:     Box<Parser>,
}

impl Engine {
    pub fn new() -> Engine {
        let u = User::new();
        Engine {
            controller: AppController::from(Box::new(SplashPage), u),
            parser: Box::new(SplashPageParser),
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
            //println!("Your command: {}", &command_string);

            // Pass the string to the parser.
            let command = (*self.parser).parse(&command_string);
           
            // Handle command.           
            match self.execute(command) {
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

    fn execute(&mut self, command: Command) -> Option<String> {
        match command {
            Command::Quit     => None,
            Command::Change(state) => {
                match self.change_state(state) {
                    Ok(_) => Some(String::from("Changing menu.")), 
                    Err(ErrorType::NotFound(e)) => {
                        Some(format!("{}", e))
                    },
                    Err(ErrorType::Duplicate(_)) => {
                        panic!("Unexpected error type.");
                    }
                }
            },
            Command::Print(s) => Some(s),
            Command::Error(s) => Some(format!("Command '{}' not recognized.", s)),        
        }
    }

    fn change_state(&mut self, state: State) -> Result<(), ErrorType> {
        match state {
            State::SplashPage => {
                self.parser = Box::new(SplashPageParser);
                self.controller.change_view(Box::new(SplashPage));  
                Ok(())              
            },
            State::MainMenu => {
                self.parser = Box::new(MainMenuParser);
                self.controller.change_view(Box::new(MainMenu));
                Ok(())
            },
            State::ClientMenu => {
                let v = self.controller.get_owned_names();
                self.parser = Box::new(ClientMenuParser);
                self.controller.change_view(Box::new(ClientMenu::from(v)));
                Ok(())                
            },
            State::TaskMenu(name) => {
                let v = self.controller.get_owned_tasks_for_client(name.clone())?;
                self.parser = Box::new(TaskMenuParser);
                self.controller.change_view(Box::new(TaskMenu::from(name, v))); 
                Ok(())                           
            },            
        }
    }
}
