use crate::bytes::read::ReadBytesAs;

impl super::MessageHeader {
    pub fn from_reader<Read: std::io::Read + std::io::Seek>(
        reader: &mut Read,
    ) -> Result<Self, std::io::Error> {
        let mut header_identifier = [0; super::consts::MESSAGE_HEADER_IDENTIFIER_LENGTH];
        match reader.read_exact(&mut header_identifier) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        if header_identifier != super::consts::MESSAGE_HEADER_IDENTIFIER {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid header identifier",
            ));
        }

        let byte_order = match reader.read_u16(crate::bytes::ByteOrder::BigEndian) {
            Ok(r) => match r {
                super::consts::LITTLE_ENDIAN_IDENTIFIER => crate::bytes::ByteOrder::LittleEndian,
                super::consts::BIG_ENDIAN_IDENTIFIER => crate::bytes::ByteOrder::BigEndian,
                _ => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Unsupported,
                        format!("Unknown byte order identifier: {r}"),
                    ))
                }
            },
            Err(e) => return Err(e),
        };
        match reader.seek(std::io::SeekFrom::Current(2)) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        let encoding = match reader.read_u8() {
            Ok(r) => match r {
                0 => crate::Encoding::Utf8,
                1 => crate::Encoding::Utf16,
                _ => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Unsupported,
                        format!("Unsupported encoding identifier: {r}"),
                    ))
                }
            },
            Err(e) => return Err(e),
        };

        match reader.seek(std::io::SeekFrom::Current(1)) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        let sections_count = match reader.read_u16(byte_order) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        let _ = match reader.seek(std::io::SeekFrom::Current(2)) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        let file_size = match reader.read_u32(byte_order) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        match reader.seek(std::io::SeekFrom::Current(10)) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        Ok(Self {
            byte_order,
            encoding,
            sections_count,
            file_size,
        })
    }
}
