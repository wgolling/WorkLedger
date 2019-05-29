use crate::app::views::{MenuView, MainMenu};
use crate::app::models::User;

pub struct AppController {
    view: Box<MenuView>,
    model: User,
}

impl AppController {
    pub fn new() -> AppController {
        AppController::from(Box::new(MainMenu::new()), User::new())
    }
    pub fn from(view: Box<MenuView>, model: User) -> AppController {
        AppController { view, model, }
    }
    pub fn display(&self) {
        (*self.view).display();
    }
    pub fn change_view(& mut self, view: Box<MenuView>) {
        self.view = view;
    }
}
