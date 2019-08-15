
extern crate chrono;
use chrono::{DateTime, Datelike, Timelike, Utc};

#[derive(Debug,Deserialize,Serialize)]
struct Record {
    client: String,
    task:   String,
    start_date: DateTime<Utc>,
    start_time: DateTime<Utc>,
    end_date:   DateTime<Utc>,
    end_time:   DateTime<Utc>,
    notes: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_working() {
        assert_eq!(1, 1);
    }
}
// struct Task {}
// impl Task {
//     pub fn new() -> Task {
//         Task {}
//     }
// }

// struct Client {
//     name: String,
// }
// impl Client {
//     pub fn new(name: String) -> Client {
//         Client {
//             name: name,
//         }
//     }
// }

// pub struct Lists {
//     clients: Vec<Client>,
// }
// impl Lists {
//     pub fn new() -> Lists {
//         Lists {
//             clients: Vec::new(),
//         }
//     }
//     pub fn new_client(& mut self, name: String) {
//         self.clients.push(Client::new(name));
//     }
// }
