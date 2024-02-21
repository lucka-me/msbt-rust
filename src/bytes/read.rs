use super::ByteOrder;

pub(crate) trait ReadBytesAs {
    fn read_u8(&mut self) -> Result<u8, std::io::Error>;
    fn read_u16(&mut self, byte_order: ByteOrder) -> Result<u16, std::io::Error>;
    fn read_u32(&mut self, byte_order: ByteOrder) -> Result<u32, std::io::Error>;
}

impl<Read: std::io::Read> ReadBytesAs for Read {
    fn read_u8(&mut self) -> Result<u8, std::io::Error> {
        let mut bytes = [0; std::mem::size_of::<u8>()];
        match self.read_exact(&mut bytes) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        Ok(bytes[0])
    }

    fn read_u16(&mut self, byte_order: ByteOrder) -> Result<u16, std::io::Error> {
        let mut bytes = [0; std::mem::size_of::<u16>()];
        match self.read_exact(&mut bytes) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        match byte_order {
            ByteOrder::BigEndian => Ok(u16::from_be_bytes(bytes)),
            ByteOrder::LittleEndian => Ok(u16::from_le_bytes(bytes)),
        }
    }

    fn read_u32(&mut self, byte_order: ByteOrder) -> Result<u32, std::io::Error> {
        let mut bytes = [0; std::mem::size_of::<u32>()];
        match self.read_exact(&mut bytes) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        match byte_order {
            ByteOrder::BigEndian => Ok(u32::from_be_bytes(bytes)),
            ByteOrder::LittleEndian => Ok(u32::from_le_bytes(bytes)),
        }
    }
}
