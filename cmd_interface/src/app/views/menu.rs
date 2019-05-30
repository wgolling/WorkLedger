pub enum MenuType {
    Main,
    Clients,
    Tasks,
    Sub,
}

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

pub struct ClientMenu<'a> {
    clients: Vec<&'a String>,
}
impl<'a> ClientMenu<'a> {
    pub fn new () -> ClientMenu<'a> {
        ClientMenu {
            clients: Vec::new(),
        }

    }
    pub fn from(clients: Vec<&'a String>) -> ClientMenu<'a> {
        ClientMenu {
            clients: clients,
        }
    }
}
impl<'a> MenuView for ClientMenu<'a> {
    fn display(&self) {
        println!("\nThis is the Clients Menu.");
        println!("Please select one of the following clients:");
    }
}

pub struct TaskMenu<'a> {
    tasks: Vec<&'a str>,
}
impl<'a> TaskMenu<'a> {
    pub fn new() -> TaskMenu<'a> {
        TaskMenu {
            tasks: Vec::new(),
        }
    }
}
impl<'a> MenuView for TaskMenu<'a> {
    fn display(&self) {
        println!("\nThis is the Tasks Menu.");
        println!("Please select one of the following tasks:");
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
    pub fn hello() -> String {
        String::from("Hello!")
    }
}

pub struct SubMenu {}
impl MenuView for SubMenu {
    fn display(&self) {
        println!("\nThis is the Sub-Menu.");
        println!("Please input a command.");
    }
}
impl SubMenu {
    pub fn new() -> SubMenu {
        SubMenu {}
    }
}
