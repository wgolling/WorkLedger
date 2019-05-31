use record_keeper::RecordKeeper;
pub use record_keeper::{ErrorType, DuplicateError, NotFoundError};


pub struct User {
    rk: RecordKeeper,
}

impl User {
    pub fn new() -> User {
        User {
            rk: RecordKeeper::new(),
        }
    }

    // pub fn validate_client_name_for_get(&self, name: String) -> Result<String, ErrorType> {
    //     let v = self.rk.get_owned_names();
    //     if !v.contains(&name) {

    //     }

    // }

    // pub fn get_owned_client_tasks(&self, client_name: String) 
    //     -> Result<Vec<String>, ErrorType> 
    // {
        
    // }


    pub fn add_client(& mut self, name: String) -> Result<(), ErrorType> {
        self.rk.add_client(name)
    }

    pub fn get_client_names(&self) -> Vec<&String> {
        self.rk.get_client_names()
    }
    pub fn get_owned_names(&self) -> Vec<String> {
        self.rk.get_owned_names()
    }
}
