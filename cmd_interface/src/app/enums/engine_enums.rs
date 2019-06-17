pub enum State {
    SplashPage,
    MainMenu,
    ClientMenu,
    TaskMenu(String),
    RecordMenu(String, String),
}

pub enum Command {
    Quit,
    Change(State),
    AddClient(String),
    AddTask(String, String),
    Print(String),
    Error(String),
}
