use std::io::prelude::*;
use std::{io::Read, net::TcpStream};
use super::packet::{self, Packet};

pub struct Client {
    password: String,
    hostname: String,
    port: u32,
    command_id: i32,
    stream: Option<TcpStream>
}

pub struct ClientConfig {
    pub password: String,
    pub hostname: String,
    pub port: u32
}

#[derive(Debug)]
pub enum ClientError {
   TcpError(std::io::Error),
   PacketError(packet::PacketError),
   NoConnectionError(NoConnectionError)
}

impl From<std::io::Error> for ClientError {
    fn from(err: std::io::Error) -> Self {
        Self::TcpError(err)
    }
}

impl From<packet::PacketError> for ClientError {
    fn from(err: packet::PacketError) -> Self {
        Self::PacketError(err)
    }
}

impl From<NoConnectionError> for ClientError {
    fn from(err: NoConnectionError) -> Self {
        Self::NoConnectionError(err)
    }
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "client error: {}", self)
    }
}

#[derive(Debug)]
pub struct NoConnectionError;

impl std::fmt::Display for NoConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "client has no TCP connection")
    }
}

impl Client {
    pub fn connect(config: ClientConfig) -> std::io::Result<Self> {
        Ok(Self{
            stream: Some(
                TcpStream::connect(
                    format!("{}:{}", config.hostname, config.port)
                )?
            ),
            password: config.password,
            hostname: config.hostname,
            port: config.port,
            command_id: 0
        })
    }

    pub fn login(mut self) -> Result<Self, ClientError> {
        let login_packet = Packet::new_auth(self.command_id, self.password.clone());
        let data = login_packet.serialize()?;
        let response = self.send(&data)?;
        let response_packet = Packet::deserialize(&response, false)?;
        if response_packet.id != self.command_id {
            println!("error logging in");
        }
        self.command_id += 1;

        Ok(self)
    }

    pub fn new(hostname: &str, port: u32, password: &str) -> Self {
        Self{ 
            password: password.to_string(),
            hostname: hostname.to_string(),
            port,
            command_id: 0,
            stream: None
        }
    }

    pub fn run(&mut self, command: &str) -> Result<String, ClientError> {
        if self.stream.is_none() {
            return Err(NoConnectionError{}.into());
        }

        let command_packet = Packet::new_command(self.command_id, command.to_string());

        let data = command_packet.serialize()?;
        let response = match self.send(&data) {
            Ok(data) => data,
            Err(why) => {
                println!("{}", why);
                return Err(packet::PacketError.into())
            }
        };

        let result_packet = Packet::deserialize(&response, false)?;
        self.command_id = (self.command_id + 1) % 1024;

        Ok(result_packet.body)
    }

    fn send(&mut self, data: &[u8]) -> Result<Vec<u8>, ClientError> {
        if let Some(stream) = &mut self.stream {
            stream.write_all(data)?;
            let mut response = Vec::<u8>::new();
            let mut buffer = [0 as u8; 128];
            let mut n = buffer.len();
            while n == buffer.len() {
                n = stream.read(&mut buffer)?;
                response.append(&mut buffer.to_vec());
            }
            return Ok(response)
        }
        
        Err(NoConnectionError.into())
    }
}
