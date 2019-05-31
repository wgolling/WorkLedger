// pub enum MenuType {
//     Main,
//     Clients,
//     Tasks,
//     Sub,
// }

pub trait MenuView {
    fn display(&self);
}


pub struct SplashPage;
impl MenuView for SplashPage {
    fn display(&self) {
        println!("\nWelcome to William's Work Ledger!\n\n");
        println!("It manages work records and compiles invoices.");
    } 
}

pub struct ClientMenu {
    client_names: Vec<String>,
}
impl ClientMenu {
    // pub fn new () -> ClientMenu {
    //     ClientMenu {
    //         client_names: Vec::new(),
    //     }

    // }
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
        // for name in &self.client_names {
        //     println!("{}", name);
        // }
    }
}

pub struct TaskMenu {
    client_name: String,
    task_names: Vec<String>,
}
impl TaskMenu {
    pub fn new() -> TaskMenu {
        TaskMenu {
            client_name: "Dummy Client".to_string(),
            task_names: Vec::new(),
        }
    }
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

pub struct MainMenu {
    // dunno what it's going to have for fields yet.
}
impl MenuView for MainMenu {
    fn display(&self) {
        println!("\nThis is the Main Menu.");
        println!("Please input a command.");
    }
}
impl MainMenu {
    pub fn new() -> MainMenu {
        MainMenu {}
    }
}



fn print_array(v: &Vec<String>) {
    for item in v {
        println!("{}", item);
    }
}
