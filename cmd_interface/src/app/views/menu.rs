use crate::app::models::User;


// Helper method for printing array.
fn print_array(v: &Vec<String>) {
    for item in v {
        println!("{}", item);
    }
}


// General view trait with display method.
pub trait MenuView {
    fn display(&self, model: &User);
}


// Splash Page View.
pub struct SplashPage;

impl MenuView for SplashPage {
    fn display(&self, model: &User) {
        println!("\nWelcome to William's Work Ledger!\n\n");
        println!("It manages work records and compiles invoices.\n\n");
        println!("Hit enter.");
    } 
}


// Main menu view.
pub struct MainMenu;

impl MenuView for MainMenu {
    fn display(&self, model: &User) {
        println!("\nThis is the Main Menu.");
        println!("Please input a command.");
    }
}


// Clients menu view.
// Displays a list of client names.
pub struct ClientMenu;
impl MenuView for ClientMenu {
    fn display(&self, model: &User) {
        println!("\nThis is the Clients Menu.");
        println!("Please select one of the following clients:");
        print_array(model.get_client_names_ref());
    }
}


// Tasks menu view.
// Displays tasks for a given client.
pub struct TaskMenu {
    client_name: String,
}

impl TaskMenu {
    pub fn from(client_name: String) -> TaskMenu {
        TaskMenu { client_name }
    }
}

impl MenuView for TaskMenu {
    fn display(&self, model: &User) {
        println!("\nThis is the Tasks Menu for {}.", &self.client_name);
        println!("Please select one of the following tasks:");
        match model.get_task_names_ref_for_client(self.client_name.clone()) {
            Ok(v) => print_array(v),
            Err(e) => panic!("TaskMenu was constructed with an invalid client."),
        };
    }
}


// Records menu view.
// Displays records for a given task.
pub struct RecordMenu {
    client_name: String,
    task_name: String,
    record_names: Vec<String>,
}

impl RecordMenu {
    pub fn from(
        client_name: String, 
        task_name: String, 
        record_names: Vec<String>
        ) -> RecordMenu 
    {
        RecordMenu{ client_name, task_name, record_names }
    }
}

impl MenuView for RecordMenu {
    fn display(&self, model: &User) {
        println!(
            "\nThis is the Records Menu for {}'s task {}.", 
            &self.client_name,
            &self.task_name
        );
        println!("Please select one of the following records:");
        print_array(&self.record_names);        
    }
}

