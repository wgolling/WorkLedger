# Work Ledger

*work in progress*

A command line interface for logging hours worked as a freelancer, and my frist Rust project. It implements a rudimentary MVC pattern for the CLI. By having the user enter records through an interface, it eliminates the risk of typos. 

At the moment only the interface has been written, I still need to implement the data serialization and retreival methods. The plan is to break the records up by months and store each month in a separate file, so that records from each month can be retrieved in parallel. Each Client and Task keeps a start and end date so that only the relevant months will be searched.


## Author

* **[William Gollinger](https://github.com/wgolling)** 


## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## Acknowledgments

* Check out [The Rust Book](https://doc.rust-lang.org/book/) to learn Rust.
