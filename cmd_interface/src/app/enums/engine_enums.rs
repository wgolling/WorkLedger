pub enum State {
    SplashPage,
    MainMenu,
    ClientMenu,
    TaskMenu,
}

pub enum Command {
    Quit,
    Change(State),
    Print(String),
    Error,
}
