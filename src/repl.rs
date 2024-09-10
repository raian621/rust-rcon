use std::error::Error;
use std::io::{stdin, stdout, Write};

use crate::rcon::client::Client;
use crate::history::History;

const PROMPT: &str = "rcon> ";

pub struct TerminalState {
    // command history; will be used to page through using the up and down arrow keys
    history: History,
}

impl TerminalState {
    fn new() -> Self {
        Self { 
            history: History::new(),
        }
    }
}

// read, evaluate, print loop for the RCON client
pub fn repl(mut client: Client) -> Result<(), Box<dyn Error>>{
    let mut input = String::new();
    let mut ts = TerminalState::new();

    while input.as_str() != "q\n" {
        input.clear();
        print!("{}", PROMPT);
        stdout().flush()?;
        stdin().read_line(&mut input)?;
        match execute(&mut client, &mut ts, &input[0..(input.len()-1)]) {
            Err(why) => println!("{}", why),
            Ok(response) => {
                if let Some(response) = response {
                    println!("{}", response);
                }
            }
        };
    }

    Ok(())
}

pub fn execute(
    client: &mut Client,
    ts: &mut TerminalState,
    cmd: &str
) -> Result<Option<String>, Box<dyn Error>> {
    ts.history.push(cmd.to_string());
    match cmd {
        "~history" => { 
            print_history(ts);
            Ok(None)
        },
        cmd => { 
            let response = client.run(cmd)?;
            if response.is_empty() {
                return Ok(None);
            }
            Ok(Some(response))
        }
    } 
}

fn print_history(ts: &TerminalState) {
    for cmd in ts.history.iter() {
        println!("{}", cmd);
    }
}
