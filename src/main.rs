pub mod rcon;

use std::{env, error::Error, io::{stdin, stdout, Write}};

use rcon::client::{Client, ClientConfig};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = eval_args(&args).unwrap();
    let mut client = Client::connect(config)
        .unwrap()
        .login()
        .unwrap();

    let mut user_input = String::new();
    while user_input != "q" {
        user_input.clear();
        print!("rcon> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut user_input).unwrap();
        let response = client.run(&user_input.as_str()[..user_input.len()-1]).unwrap();
        if response.len() > 0 {
            println!("{}", response);
        }
    }
    println!("Hello, world!");
}

// evaluate command line arguments, loading them into a ServerConfig object
fn eval_args(args: &[String]) -> Result<ClientConfig, Box<dyn Error>> {
    if args.len() < 2 {
        return Err("invalid number of arguments".into());
    }
    println!("{:?}", args);

    let mut hostname = None;
    let mut port = None;
    let mut password = None;

    let mut curr = 1;
    while curr < args.len() {
        match args[curr].as_str() {
            "-h"|"--hostname" => {
                curr += 1;
                if curr < args.len() {
                    hostname = Some(args[curr].clone())
                }
            },
            "-p"|"--port" => {
                curr += 1;
                if curr < args.len() {
                    port = Some(args[curr].parse::<u32>()?);
                }
            }
            "-P"|"--password" => {
                curr += 1;
                if curr < args.len() {
                    password = Some(args[curr].clone());
                }
            }
            _ => {
                return Err(format!("invalid arg at index {}", curr).into());
            }
        }
        curr += 1;
    }

    if hostname.is_none() {
        return Err("hostname not provided (use `-h <hostname>` or `--hostname <hostname>`".into());
    }
    if port.is_none() {
        return Err("port not provided (use `-p <port>` or `--port <port>`".into());
    }
    if password.is_none() {
        let mut user_input = String::new();

        print!("RCON password: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut user_input).unwrap();

        password = Some(user_input[..user_input.len()-1].to_string());
    }

    Ok(ClientConfig{
        hostname: hostname.unwrap(),
        port: port.unwrap(),
        password: password.unwrap()
    })
}
