use std::fs::File;
use std::io::{self,Read,ErrorKind};
use std::fs;
use std::error::Error;
use std::net::IpAddr;

fn main() {
    {
        let greeting_file_result = File::open("hello.txt");
        let _gretting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error)
                }
            },
        };
    }

    // Alternatives
    {
        let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file:{:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
    }

    //Simpler!
    {
        let _greeting_file = File::open("hello.txt").unwrap();
        // This will just crash on errors

    }

    {
        let _greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");
    }
    
    // propagating errors

    {
        fn read_username_from_file() -> Result<String,io::Error> {
            let username_file_result = File::open("hello.txt");

            let mut username_file = match username_file_result {
                Ok(file) => file,
                Err(e) => return Err(e)
            };

            let mut username = String::new();

            match username_file.read_to_string(&mut username) {
                Ok(_) => Ok(username),
                Err(e) => Err(e),
            }
        }
    }

    // EVEN SIMPLER
    {
        fn read_username_from_file() -> Result<String, io::Error> {
            let mut username_file = File::open("hello.txt")?;
            let mut username = String::new();
            username_file.read_to_string(&mut username)?;
            Ok(username)
        }
    }

    {
        // can be even shorter
        fn read_username_from_file() -> Result<String, io::Error> {
            let mut username = String::new();
            File::open("hello.txt")?.read_to_string(&mut username)?;
            Ok(username)
        }
    }

    {
        // shorter?!
        fn read_username_from_file() -> Result<String, io::Error> {
            fs::read_to_string("hello.txt")
        }
    }

    // The ? operator cannot be used in functions that do not return a Result type with errors, or
    // an Option<?> type

    {
        fn last_char_of_first_line(text: &str) -> Option<char> {
            text.lines().next()?.chars().last()
        }
    }

    // In order to use ? in main, you can change the return of main:
    {
        fn main_example() -> Result<(), Box<dyn Error>> {
            let greeting_file = File::open("hello.txt")?;

            Ok(())
        }
    }

    {
        let home: IpAddr = "127.0.0.1"
            .parse()
            .expect("Hardcoded IP address should be valid");
    }
}
