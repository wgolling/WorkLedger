use std::collections::HashMap;
use std::error::Error;
//use std::collections::HashSet;

mod record;

mod error;
use error::{DuplicateError, NotFoundError};


struct Task<'a> {
    name: &'a str,
}
impl<'a> Task<'a> {
    pub fn new(name: &'a str) -> Task<'a> {
        Task {
            name: name,
        }
    }
}

struct Client<'a> {
    name: &'a str,
    tasks: HashMap<&'a str, Task<'a>>,
}
impl<'a> Client<'a> {
    pub fn new(name: &'a str) -> Client<'a> {
        Client {
            name: name,
            tasks: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> &'a str {
        self.name
    }

    pub fn add_task(& mut self, name: &'a str) -> Result<(), Box<dyn Error>> {
        match self.tasks.get(name) {
            Some(_) => Err(Box::new(DuplicateError)),
            None    => {
                self.tasks.insert(name, Task::new(name));
                Ok(())
            }
        }
    }

    pub fn get_task_names(&self) -> Vec<&'a str> {
        let mut result = Vec::new();
        for &name in self.tasks.keys() {
            result.push(name);
        }
        result.sort();
        result
    }
}

pub struct RecordKeeper<'a> {
    //lists: record::Lists,
    clients: HashMap<&'a str, Client<'a>>,
}
impl<'a> RecordKeeper<'a> {
    pub fn new() -> RecordKeeper<'a> {
        RecordKeeper {
            //lists: record::Lists::new(),
            clients: HashMap::new(),
        }
    }
    pub fn add_client(& mut self, name: &'a str) -> Result<(), Box<dyn Error>> {
        match self.clients.get(name) {
            Some(_) => Err(Box::new(DuplicateError)),
            None    => {
                self.clients.insert(name, Client::new(name));
                Ok(())
            }
        }
    }
    pub fn get_client_names(&self) -> Vec<&'a str> {
        let mut result = Vec::new();
        for &name in self.clients.keys() {
            result.push(name);
        }
        result.sort();
        result
    }

    pub fn add_task(& mut self, client_name: &'a str, task_name: &'a str)
        -> Result<(), Box<dyn Error>>
    {
        let mut client = self
            .clients
            .entry(client_name)
            .or_insert(Client::new(client_name));
        client.add_task(task_name)
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    // Testing Task
    #[test]
    fn make_task() {
        let task = Task::new("Test Task");
        assert_eq!(task.name, "Test Task");
    }

    //Testing Client
    #[test]
    fn make_client() {
        let client = Client::new("Test Client");
        assert_eq!(client.name, "Test Client");
        assert_eq!(client.get_task_names().len(), 0);
    }
    #[test]
    fn add_task_to_client() {
        let mut client = Client::new("Test Client");
        client.add_task("Test Task");
        client.add_task("Test Task 2");
        assert_eq!(client.get_task_names(), [ "Test Task", "Test Task 2" ]);
        assert!(client.add_task("Test Task").is_err());
    }


    // Testing RecordKeeper
    #[test]
    fn make_new_record_keeper() {
        let rk = RecordKeeper::new();
    }

    #[test]
    fn add_clients() {
        let mut rk = RecordKeeper::new();
        rk.add_client("Test Client");
        rk.add_client("Test Client 2");
        assert!(rk.add_client("Test Client").is_err());
    }

    #[test]
    fn get_client_names() {
        let mut rk = RecordKeeper::new();
        rk.add_client("Test Client");
        rk.add_client("Test Client 2");
        let list = rk.get_client_names();

    }
}







