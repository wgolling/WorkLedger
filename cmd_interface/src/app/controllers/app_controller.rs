use crate::app::views::{MenuView, SplashPage};
use crate::app::models::User;
use record_keeper::ErrorType;

pub struct AppController {
    view: Box<MenuView>,
    model: User,
}

impl AppController {
    // pub fn new() -> AppController {
    //     AppController::from(Box::new(SplashPage), User::new())
    // }
    pub fn from(view: Box<MenuView>, model: User) -> AppController {
        AppController { view, model, }
    }
    pub fn display(&self) {
        (*self.view).display();
    }
    pub fn change_view(& mut self, view: Box<MenuView>) {
        self.view = view;
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
}
