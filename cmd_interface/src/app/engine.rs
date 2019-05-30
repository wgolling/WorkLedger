use std::io;

use super::parser::*;
use super::controllers::AppController;
use super::views::*;
use super::enums::*;


pub struct Engine {
    controller: AppController,
    parser:     Box<Parser>,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            controller: AppController::new(),
            parser:     Box::new(SplashPageParser),
        }
    }

    pub fn run(& mut self) {
        loop {
            // Display menu.
            self.display();

            // Try to store user input in a String.
            let mut command = String::new();
            io::stdin().read_line(&mut command)
                .expect("Failed to read line.");

            command = command.trim().to_string();

            println!("Your command: {}", &command);

            // Pass the command to the parser.
            let result = self.execute((*self.parser).parse(command));
            match result {
                Some(s) => println!("{}", s),
                None    => break,
            }

            // End of loop.
        }

    }

    pub fn display(&self) {
        self.controller.display();
    }

    fn execute(&mut self, command: Command) -> Option<String> {
        match command {
            Command::Quit     => None,
            Command::Change(state)   
                              => {
                self.change_state(state);
                return Some(String::from("Changing menu."))
            }
            Command::Print(s) => Some(s),
            Command::Error    => Some(String::from("Command not recognized.")),        
        }
    }

    fn change_state(&mut self, state: State) {
        match state {
            State::SplashPage => {
                self.parser = Box::new(SplashPageParser);
                self.controller.change_view(Box::new(SplashPage));                
            },
            State::MainMenu => {
                self.parser = Box::new(MainMenuParser);
                self.controller.change_view(Box::new(MainMenu::new()));
            },
            State::ClientMenu => {
                self.parser = Box::new(ClientMenuParser);
                self.controller.change_view(Box::new(ClientMenu::new()));                
            },
            State::TaskMenu => {
                self.parser = Box::new(TaskMenuParser);
                self.controller.change_view(Box::new(TaskMenu::new()));                
            },            
        }
    }
}
