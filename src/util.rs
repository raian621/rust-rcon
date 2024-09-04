use std::{env, error::Error};

use crate::rcon::client::ClientConfig;

// evaluate command line arguments, loading them into a ServerConfig object
pub fn eval_args(args: &[String]) -> Result<ClientConfig, Box<dyn Error>> {
    let mut hostname = match env::var("RCON_HOSTNAME") {
        Err(_) => None,
        Ok(env_var) => Some(env_var)
    };
    let mut port = match env::var("RCON_PORT") {
        Err(_) => None,
        Ok(env_var) => Some(env_var.parse::<u16>()?)
    };
    let mut password = match env::var("RCON_PASSWORD") {
        Err(_) => None,
        Ok(env_var) => Some(env_var)
    };

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
                    port = Some(args[curr].parse::<u16>()?);
                }
            }
            "-P"|"--password" => {
                curr += 1;
                if curr < args.len() {
                    password = Some(args[curr].clone());
                }
            }
            _ => {
                return Err(format!("invalid arg `{}`", args[curr]).into());
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
        return Err("password not provided (use `-P <password>` or `--password <password>`".into());
    }

    Ok(ClientConfig{
        hostname: hostname.unwrap(),
        port: port.unwrap(),
        password: password.unwrap()
    })
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
  
    // sets environment variables and returns old environment variables
    fn set_env_vars(
        hostname: Option<String>,
        port: Option<String>,
        password: Option<String>
    ) -> (Option<String>, Option<String>, Option<String>) {
        let old_hostname = match env::var("RCON_HOSTNAME") {
            Err(_) => None,
            Ok(env_var) => Some(env_var)
        };
        let old_port = match env::var("RCON_PORT") {
            Err(_) => None,
            Ok(env_var) => Some(env_var)
        };
        let old_password = match env::var("RCON_PASSWORD") {
            Err(_) => None,
            Ok(env_var) => Some(env_var)
        };
        
        if let Some(hostname) = hostname {
            env::set_var("RCON_HOSTNAME", hostname);
        }
        if let Some(port) = port {
            env::set_var("RCON_PORT", port);
        }
        if let Some(password) = password {
            env::set_var("RCON_PASSWORD", password);
        }

        (old_hostname, old_port, old_password)
    }

    #[test]
    fn test_eval_args() {
        let args: Vec<String> = vec!["rust-rcon", "-P", "password", "-h", "minecraft.lan", "-p", "25575"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        if let Ok(config) = eval_args(&args) {
            assert_eq!(config.hostname, "minecraft.lan".to_string());
            assert_eq!(config.port, 25575);
            assert_eq!(config.password, "password");
        } else {
            assert!(false);
        }
        let args: Vec<String> = vec![
            "rust-rcon",
            "--password", "password",
            "--hostname", "minecraft.lan",
            "--port", "25575"
        ]
            .iter()
            .map(|x| x.to_string())
            .collect();
        if let Ok(config) = eval_args(&args) {
            assert_eq!(config.hostname, "minecraft.lan".to_string());
            assert_eq!(config.port, 25575);
            assert_eq!(config.password, "password");
        } else {
            assert!(false);
        }

        let (old_hostname, old_port, old_password) = set_env_vars(
            Some("minecraft.lan".to_string()),
            Some("25575".to_string()),
            Some("password".to_string())
        );

        match eval_args(&vec!["rust-rcon".to_string()]) {
            Ok(config) => {
                assert_eq!(config.hostname, "minecraft.lan".to_string());
                assert_eq!(config.port, 25575);
                assert_eq!(config.password, "password");
            },
            Err(why) => {
                println!("{}", why);
                assert!(false);
            }
        }

        set_env_vars(old_hostname, old_port, old_password);
    }

    #[test]
    fn test_invalid_arguments() {
        // invalid flag
        let args = vec!["rust-rcon".to_string(), "-r".to_string()];

        if let Err(why) = eval_args(&args) {
            assert_eq!(why.to_string(), "invalid arg `-r`");
        } else {
            println!("expected argument error");
            assert!(false);
        }

        // invalid port (not a number)
        let args = vec![
            "rust-rcon".to_string(), 
            "-h".to_string(), "minecraft.lan".to_string(),
            "-P".to_string(), "password".to_string(),
            "-p".to_string(), "notanumber".to_string(),
        ];
        if let Err(why) = eval_args(&args) {
            assert_eq!(why.to_string(), "invalid digit found in string");
        } else {
            println!("expected integer parsing error");
            assert!(false);
        }

        // invalid port (negative)
        let args = vec![
            "rust-rcon".to_string(), 
            "-h".to_string(), "minecraft.lan".to_string(),
            "-P".to_string(), "password".to_string(),
            "-p".to_string(), "-1".to_string(),
        ];
        if let Err(why) = eval_args(&args) {
            assert_eq!(why.to_string(), "invalid digit found in string");
        } else {
            println!("expected integer parsing error");
            assert!(false);
        }

        // invalid port (too big)
        let args = vec![
            "rust-rcon".to_string(), 
            "-h".to_string(), "minecraft.lan".to_string(),
            "-P".to_string(), "password".to_string(),
            "-p".to_string(), "1000000".to_string(),
        ];
        if let Err(why) = eval_args(&args) {
            assert_eq!(why.to_string(), "number too large to fit in target type");
        } else {
            println!("expected integer parsing error");
            assert!(false);
        }
    }
}


