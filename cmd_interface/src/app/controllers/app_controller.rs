use crate::app::views::MenuView;
use crate::app::models::User;

pub struct AppController {
    view: Box<MenuView>,
    model: User,
}

impl AppController {
    fn new(view: Box<MenuView>, model: User) -> AppController {
        AppController { view, model, }
    }
    pub fn display(&self) {
        self.view.display();
    }
    fn change_view(& mut self, view: Box<MenuView>) {
        self.view = view;
    }
}
