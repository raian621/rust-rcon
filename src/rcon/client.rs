use std::io::prelude::*;
use std::{io::Read, net::TcpStream};
use super::packet::{self, Packet};

pub struct Client {
    password: String,
    hostname: String,
    port: u16,
    command_id: i32,
    stream: Option<TcpStream>
}

pub struct ClientConfig {
    pub password: String,
    pub hostname: String,
    pub port: u16
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
        match self {
            ClientError::NoConnectionError(err) => write!(f, "client error: {}", err),
            ClientError::TcpError(err) => write!(f, "client error: {}", err),
            ClientError::PacketError(err) => write!(f, "client error: {}", err),
        }
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
    // connect to an RCON game server
    pub fn connect(hostname: String, port: u16, password: String) -> Result<Client, ClientError> {
        let mut client = Self{
            stream: None,
            hostname,
            password,
            port,
            command_id: 0,
        };

        // connect to RCON port on target server
        client.stream = Some(TcpStream::connect(
            format!("{}:{}", client.hostname, client.port)
        )?);

        // authenticate with the server using the RCON password
        client.login()?;

        Ok(client)
    }

    // authenticate with the RCON server using the RCON password
    fn login(&mut self) -> Result<(), ClientError> {
        let login_packet = Packet::new_auth(self.command_id, self.password.clone());
        let data = login_packet.serialize()?;
        let response = self.send(&data)?;
        let response_packet = Packet::deserialize(&response, false)?;
        if response_packet.id != self.command_id {
            println!("error logging in");
        }
        self.command_id += 1;

        Ok(())
    }

    // send a command to the RCON server and return its response
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

    // send binary data to the RCON game server and return its response
    fn send(&mut self, data: &[u8]) -> Result<Vec<u8>, ClientError> {
        if let Some(stream) = &mut self.stream {
            stream.write_all(data)?;
            let mut response = Vec::<u8>::new();
            let mut buffer = [0_u8; 128];
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
