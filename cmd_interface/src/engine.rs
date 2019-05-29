use std::{io, process};

use record_keeper::RecordKeeper;
use record_keeper::error::*;

//mod menu;
use crate::menu::menu::*;


// Engine should implement the MVC model.
// The Model should wrap RecordKeeper, like UserModel or something.
// The Views are Menus, which depend on a State object (maybe an Enum).
// The State should also determine an input parser.
struct UserModel {
    rk: RecordKeeper,
}
impl UserModel {
    fn new() -> UserModel {
        UserModel {
            rk: RecordKeeper::new(),
        }
    }
}

struct AppController {
    view: Box<MenuView>,
    model: UserModel,
}
impl AppController {
    fn new(view: Box<MenuView>, model: UserModel) -> AppController {
        AppController { view, model, }
    }
    pub fn display(&self) {
        self.view.display();
    }
    fn change_view(& mut self, menu: MenuType) {
        self.view = match menu {
            MenuType::Main    => Box::new(MainMenu::new()),
            MenuType::Clients => Box::new(Clients::new()),
            MenuType::Tasks   => Box::new(Tasks::new()),
            MenuType::Sub     => Box::new(SubMenu::new()),
        };
    }
}

pub struct Engine {
    menu: Box<MenuView>,
    user: UserModel,
    controller: AppController,
}
impl Engine {
    pub fn new() -> Engine {
        Engine {
            menu: Box::new(MainMenu::new()),
            user: UserModel::new(),
            controller: AppController::new(Box::new(MainMenu::new()), UserModel::new()),
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

            // Pass the command to the menu engine.
            match self.parse_command(command) {
                Some(s) => println!("{}", s),
                None    => break,
            }

            // End of loop.
        }

    }

    pub fn display(&self) {
        self.controller.display();
    }
    pub fn parse_command(&mut self, command: String) -> Option<String> {
        match (*self.menu).parse_command(command) {
            Command::Quit     => None,
            Command::Change(menu_type)   
                              => {
                self.change_menu(menu_type);
                return Some(String::from("Changing menu."))
            }
            Command::Print(s) => Some(s),
            Command::Error    => Some(String::from("Command not recognized.")),        
        }
    }
    fn change_menu(& mut self, menu: MenuType) {
        self.controller.change_view(menu);
    }
}
