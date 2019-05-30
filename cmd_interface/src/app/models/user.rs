use record_keeper::RecordKeeper;

pub struct User {
    rk: RecordKeeper,
}

impl User {
    pub fn new() -> User {
        User {
            rk: RecordKeeper::new(),
        }
    }
    pub fn get_client_names(&self) -> Vec<&String> {
        self.rk.get_client_names()
    }
}
