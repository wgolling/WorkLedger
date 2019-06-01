use crate::app::views::{MenuView, SplashPage};
use crate::app::models::User;
use record_keeper::ErrorType;


// General app controller.
// Wraps together the interfaces for the model and view.
pub struct AppController {
    view: Box<MenuView>,
    model: User,
}

impl AppController {
    pub fn from(view: Box<MenuView>, model: User) -> AppController {
        AppController { view, model, }
    }

    // Displaying view.
    pub fn display(&self) {
        (*self.view).display();
    }

    // Changing view.
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
