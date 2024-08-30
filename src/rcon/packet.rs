pub type Result<T> = std::result::Result<T, PacketError>;

#[derive(Debug, Clone)]
pub struct PacketError;

impl std::fmt::Display for PacketError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "error parsing or serializing RCON packet")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PacketType {
    ServerDataAuth,
    ServerDataAuthResponse,
    ServerDataExecCommand,
    ServerDataResponseValue
}

#[derive(Debug)]
pub struct Packet {
    pub id: i32,
    pub packet_type: PacketType,
    pub body: String,
}

impl Packet {
    pub fn serialize(&self) -> Result<Vec<u8>> {
        let size: i32 = self.body.len() as i32 + 10;
        let packet_type = match self.packet_type {
            PacketType::ServerDataAuth => 3,
            PacketType::ServerDataAuthResponse => 2,
            PacketType::ServerDataExecCommand => 2,
            PacketType::ServerDataResponseValue => 0,
        };

        let mut buffer = vec![0 as u8; size as usize + 4];
        
        // write the packet size into the buffer (little endian)
        for i in 0..4 {
            buffer[i] = (size >> i*8) as u8;
        }

        // write the packet id into the buffer
        for i in 0..4 {
            buffer[i+4] = (self.id >> i*8) as u8;
        }

        // write the packet type into the buffer
        for i in 0..4 {
            buffer[i+8] = (packet_type >> i*8) as u8;
        }

        // write body into buffer
        let mut curr = 12;
        for i in 0..self.body.len() {
            buffer[curr] = self.body.chars().nth(i).unwrap() as u8;
            curr += 1;
        }

        Ok(buffer)
    }

    pub fn deserialize(data: &[u8], request: bool) -> Result<Self> {
        let size = 
            data[0] as i32 |
            (data[1] as i32) << 0x08 |
            (data[2] as i32) << 0x10 |
            (data[3] as i32) << 0x18;
        let id  = 
            data[4] as i32 |
            (data[5] as i32) << 0x08 |
            (data[6] as i32) << 0x10 |
            (data[7] as i32) << 0x18;
        let packet_type_bits = 
            data[8] as i32 |
            (data[9] as i32) << 0x08 |
            (data[10] as i32) << 0x10 |
            (data[11] as i32) << 0x18;

        let packet_type = match packet_type_bits {
            0 => PacketType::ServerDataResponseValue,
            2 => { 
                if request { 
                    PacketType::ServerDataExecCommand 
                } else { 
                    PacketType::ServerDataResponseValue 
                }
            },
            3 => PacketType::ServerDataAuth,
            _ => return Err(PacketError)
        };

        let body = String::from_utf8(data[12..size as usize + 2].to_vec()).unwrap();

        Ok(Self { id, packet_type, body })
    }

    pub fn new_command(id: i32, command: String) -> Self {
        Self { id, body: command, packet_type: PacketType::ServerDataExecCommand }
    }

    pub fn new_auth(id: i32, password: String) -> Self {
        Self { id, body: password, packet_type: PacketType::ServerDataAuth }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_and_deserialization() {
        let packet = Packet{
            id: 12,
            packet_type: PacketType::ServerDataExecCommand,
            body: "give @a effect speed 2 20".to_string()
        };

        let data = packet.serialize().unwrap();

        println!("{:?}", data);
        let deserialized = Packet::deserialize(&data, true).unwrap();

        assert_eq!(packet.id, deserialized.id);
        assert_eq!(packet.packet_type, deserialized.packet_type);
        assert_eq!(packet.body, deserialized.body);
        assert!(false);
    }
}
