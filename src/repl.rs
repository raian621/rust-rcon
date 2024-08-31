use std::io::{stdin, stdout, Write};

use crate::rcon::client::Client;

// read, evaluate, print loop for the RCON client
pub fn repl(mut client: Client) {
    let mut user_input = String::new();
    while user_input != "q" {
        user_input.clear();
        print!("rcon> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut user_input).unwrap();
        let response = client.run(&user_input.as_str()[..user_input.len()-1]).unwrap();
        if !response.is_empty() {
            println!("{}", response);
        }
    }
}

