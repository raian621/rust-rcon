pub mod rcon;
pub mod repl;
pub mod util;

use std::{env, process::exit};

use rcon::client::Client;
use repl::repl;
use util::eval_args;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = match eval_args(&args) {
        Ok(config) => config,
        Err(why) => {
            println!("{}", why);
            exit(1);
        }
    };
    let client = Client::connect(
        config.hostname,
        config.port,
        config.password
    ).unwrap();

    repl(client);
}

