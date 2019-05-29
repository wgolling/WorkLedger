pub enum MenuType {
    Main,
    Clients,
    Tasks,
    Sub,
}

pub trait MenuView {
    fn display(&self);
}

pub struct Clients<'a> {
    clients: Vec<&'a str>,
}
impl<'a> Clients<'a> {
    pub fn new() -> Clients<'a> {
        Clients {
            clients: Vec::new(),
        }
    }
}
impl<'a> MenuView for Clients<'a> {
    fn display(&self) {
        println!("\nThis is the Clients Menu.");
        println!("Please select one of the following clients:");
    }
}

pub struct Tasks<'a> {
    tasks: Vec<&'a str>,
}
impl<'a> Tasks<'a> {
    pub fn new() -> Tasks<'a> {
        Tasks {
            tasks: Vec::new(),
        }
    }
}
impl<'a> MenuView for Tasks<'a> {
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
