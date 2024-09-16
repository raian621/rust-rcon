pub mod rcon;
pub mod repl;
pub mod history;

use std::process::exit;
use std::env;
use clap::Parser;

use rcon::client::Client;
use repl::repl;

#[derive(Parser)]
#[command(version = "0.2.0", about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "CMD")]
    adhoc: Option<String>,
    #[arg(short = 'H', long)]
    hostname: Option<String>,
    #[arg(short = 'P', long)]
    port: Option<u16>,
    #[arg(short = 'p', long)]
    password: Option<String>
}

fn main() {
    let mut args = Args::parse();
    load_env_args(&mut args);
    if let Err(why) = required_args_present(&args) {
        print!("{}", why);
        exit(1);
    }
    let mut client = Client::connect(
        args.hostname.unwrap(),
        args.port.unwrap(),
        args.password.unwrap()
    ).unwrap();

    match args.adhoc {
        None => println!("{:?}", repl(client)),
        Some(cmd) => {
            match client.run(cmd.as_str()) {
                Ok(response) => println!("{:?}", response),
                Err(why) => {
                    println!("Unexpected error occurred: {:?}", why);
                    exit(1);
                }
            }
        }
    };
}

fn load_env_args(args: &mut Args) {
    if args.hostname.is_none() {
        args.hostname = match env::var("RCON_HOSTNAME") {
            Ok(hostname) => Some(hostname),
            Err(_) => None
        }
    }
    if args.port.is_none() {
        args.port = match env::var("RCON_PORT") {
            Ok(port) => Some(port.parse::<u16>().unwrap()),
            Err(_) => None
        }
    }
    if args.password.is_none() {
        args.password = match env::var("RCON_PASSWORD") {
            Ok(password) => Some(password),
            Err(_) => None
        }
    }
}

fn required_args_present(args: &Args) -> Result<(), String> {
    let mut args_absent = 0;

    if args.hostname.is_none() {
        args_absent |= 1;
    }
    if args.port.is_none() {
        args_absent |= 2;
    }
    if args.password.is_none() {
        args_absent |= 4;
    }
    
    match args_absent {
        0b001 => Err("hostname argument not provided".to_string()),
        0b010 => Err("port argument not provided".to_string()),
        0b011 => Err("hostname and port arguments not provided".to_string()),
        0b100 => Err("password argument not provided".to_string()),
        0b101 => Err("hostname and password arguments not provided".to_string()),
        0b110 => Err("hostname and port argument not provided".to_string()),
        0b111 => Err("hostname, port, and password arguments not provided".to_string()),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_env_args() {
        let hostname = "minecraft.lan".to_string();
        let port = 25575_u16;
        let password = "very secure password".to_string();

        env::set_var("RCON_HOSTNAME", hostname.clone());
        env::set_var("RCON_PORT", port.to_string());
        env::set_var("RCON_PASSWORD", password.clone());

        let mut args = Args {
            adhoc: None,
            hostname: None,
            port: None,
            password: None,
        };

        load_env_args(&mut args);
        assert_eq!(args.adhoc, None);
        assert_eq!(args.hostname, Some(hostname));
        assert_eq!(args.port, Some(port));
        assert_eq!(args.password, Some(password));
    }

    #[test]
    fn test_load_env_args_cli_argument_precedence() {
        let env_hostname = "env_minecraft.lan".to_string();
        let env_password = "env_very secure password".to_string();
        let env_port = 42069;
        env::set_var("RCON_HOSTNAME", env_hostname.clone());
        env::set_var("RCON_PORT", env_port.to_string());
        env::set_var("RCON_PASSWORD", env_password.clone());
        let hostname = "minecraft.lan".to_string();
        let port = 25575_u16;
        let password = "very secure password".to_string();

        let mut args = Args {
            adhoc: None,
            hostname: Some(hostname.clone()),
            port: Some(port),
            password: Some(password.clone()),
        };

        load_env_args(&mut args);
        assert_eq!(args.adhoc, None);
        assert_eq!(args.hostname, Some(hostname));
        assert_eq!(args.port, Some(port));
        assert_eq!(args.password, Some(password));
    }
    
    #[test]
    fn test_required_args_present() {
        assert_eq!(required_args_present(&Args{
            adhoc: None,
            hostname: Some("d".to_string()),
            port: Some(420),
            password: Some("d".to_string()),
        }), Ok(()));
        assert_eq!(required_args_present(&Args{
            adhoc: None,
            hostname: None, 
            port: Some(420),
            password: Some("d".to_string()),
        }), Err("hostname argument not provided".to_string()));
        assert_eq!(required_args_present(&Args{
            adhoc: None,
            hostname: None, 
            port: None,
            password: None,
        }), Err("hostname, port, and password arguments not provided".to_string()));
    }
}
