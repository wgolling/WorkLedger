// General view trait with display method.
pub trait MenuView {
    fn display(&self);
}


// Helper method for printing arrary.
fn print_array(v: &Vec<String>) {
    for item in v {
        println!("{}", item);
    }
}


// Splash Page View.
pub struct SplashPage;

impl MenuView for SplashPage {
    fn display(&self) {
        println!("\nWelcome to William's Work Ledger!\n\n");
        println!("It manages work records and compiles invoices.");
    } 
}


// Main menu view.
pub struct MainMenu;

impl MenuView for MainMenu {
    fn display(&self) {
        println!("\nThis is the Main Menu.");
        println!("Please input a command.");
    }
}


// Clients menu view.
// Displays a list of client names.
pub struct ClientMenu {
    client_names: Vec<String>,
}

impl ClientMenu {
    pub fn from(client_names: Vec<String>) -> ClientMenu {
        ClientMenu {
            client_names: client_names,
        }
    }
}

impl MenuView for ClientMenu {
    fn display(&self) {
        println!("\nThis is the Clients Menu.");
        println!("Please select one of the following clients:");
        print_array(&self.client_names);
    }
}


// Tasks menu view.
// Displays tasks for a given client.
pub struct TaskMenu {
    client_name: String,
    task_names: Vec<String>,
}

impl TaskMenu {
    pub fn from(client_name: String, task_names: Vec<String>) -> TaskMenu {
        TaskMenu { client_name, task_names }
    }
}

impl MenuView for TaskMenu {
    fn display(&self) {
        println!("\nThis is the Tasks Menu for {}.", &self.client_name);
        println!("Please select one of the following tasks:");
        print_array(&self.task_names);
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
    fn display(&self) {
        println!(
            "\nThis is the Records Menu for {}'s task {}.", 
            &self.client_name,
            &self.task_name
        );
        println!("Please select one of the following records:");
        print_array(&self.record_names);        
    }
}

