use crate::app::{
    views::*, 
    models::User,
    enums::{State, Command},
    parser::*,
};
use record_keeper::ErrorType;


// General app controller.
// Wraps together the interfaces for the model and view.
pub struct AppController {
    view: Box<MenuView>,
    model: User,
    parser: Box<Parser>,
}

impl AppController {
    pub fn from(
        view: Box<MenuView>, 
        model: User, 
        parser: Box<Parser>,
        ) -> AppController 
    {
        AppController { view, model, parser, }
    }

    // Processing input.
    pub fn process_command(&mut self, command_string: String) -> Option<String> {
        self.execute(self.parser.parse(&command_string))
    }

    fn execute(&mut self, command: Command) -> Option<String> {
        match command {
            Command::Quit     => None,
            Command::AddClient(client_name) 
                              => {
                match self.model.add_client(client_name.clone()) {
                    Ok(_) => Some(format!("Adding client {}", client_name)),
                    Err(ErrorType::Duplicate(e)) => {
                        Some(format!("{}", e))                        
                    },
                    Err(ErrorType::NotFound(_)) => {
                        panic!("Unexpected error type.");
                    }
                }
            },
            Command::AddTask(client_name, task_name) 
                              => {
                match self.model.add_task(client_name.clone(), task_name.clone()) {
                    Ok(_) => Some(format!("Adding task {} for client {}.", task_name, client_name)),
                    Err(ErrorType::Duplicate(e)) => {
                        Some(format!("{}", e))                        
                    },
                    Err(ErrorType::NotFound(e)) => {
                        Some(format!("{}", e))                        
                    }
                }
            },
            Command::Change(state) 
                              => {
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

    // Changing state.
    fn change_state(&mut self, state: State) -> Result<(), ErrorType> {
        match state {
            State::SplashPage => {
                self.change_parser(Box::new(SplashPageParser));
                self.change_view(Box::new(SplashPage));  
            },
            State::MainMenu => {
                self.change_parser(Box::new(MainMenuParser));
                self.change_view(Box::new(MainMenu));
            },
            State::ClientMenu => {
                let v = self.get_owned_names();
                self.change_parser(Box::new(ClientMenuParser::from(v.clone())));
                self.change_view(Box::new(ClientMenu {} ));
            },
            State::TaskMenu(name) => {
                let v = self.get_owned_tasks_for_client(name.clone())?;
                self.change_parser(Box::new(
                    TaskMenuParser::from(name.clone(), v.clone())
                ));
                self.change_view(Box::new(TaskMenu::from(name))); 
            },  
            State::RecordMenu(client_name, task_name) => {
                let records = vec![
                    "1".to_string(), 
                    "2".to_string(), 
                    "3".to_string()
                ];
                self.change_parser(Box::new(RecordMenuParser::from(
                    client_name.clone(), 
                    task_name.clone(),
                    records.clone()
                )));
                self.change_view(Box::new(RecordMenu::from(
                    client_name, task_name, records
                )));
            }, 
            _ => panic!("State not recognized"),      
        }
        Ok(())              

    }


    // Displaying view.
    pub fn display(&self) {
        (*self.view).display(&self.model);
    }

    // Changing state.
    pub fn change_view(& mut self, view: Box<MenuView>) {
        self.view = view;
    }
    pub fn change_parser(& mut self, parser: Box<Parser>) {
        self.parser = parser;
    }

    // Adding Clients
    pub fn add_client(& mut self, name: String) -> Result<(), ErrorType> {
        self.model.add_client(name)
    }

    // Getting client names.
    pub fn get_client_names(&self) -> Vec<&String> {
        self.model.get_client_names()
    }
    pub fn get_owned_names(&self) -> Vec<String> {
        self.model.get_owned_names()
    }

    // Adding Tasks to clients.
    pub fn add_task(& mut self, client_name: String, task_name: String)
        -> Result<(), ErrorType>
    {
        self.model.add_task(client_name, task_name)
    }

    // Getting tasks for clients.
    pub fn get_owned_tasks_for_client(&self, client_name: String) 
        -> Result<Vec<String>, ErrorType> 
    {
        self.model.get_owned_tasks_for_client(client_name)
    }
}
