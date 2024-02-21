use crate::bytes::read::ReadBytesAs;

impl super::AttributeSection {
    pub(super) fn from_reader<Read: std::io::Read + std::io::Seek>(
        reader: &mut Read,
        byte_order: crate::bytes::ByteOrder,
    ) -> Result<Self, std::io::Error> {
        let header = match super::Header::from_reader(reader, byte_order) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        for _ in 0 .. header.entries_count {
        }

        match reader.seek(std::io::SeekFrom::Start(header.body_start_position + u64::from(header.body_size))) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        match seek_section_end(reader) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        Ok(Self { })
    }
}

impl super::LabelSection {
    pub(super) fn from_reader<Read: std::io::Read + std::io::Seek>(
        reader: &mut Read,
        byte_order: crate::bytes::ByteOrder,
    ) -> Result<Self, std::io::Error> {
        let header = match super::Header::from_reader(reader, byte_order) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        let mut labels = std::vec::Vec::<std::vec::Vec<super::Label>>::new();
        for _ in 0 .. header.entries_count {
            let count = match reader.read_u32(byte_order) {
                Ok(r) => r,
                Err(e) => return Err(e),
            };
            let offset: u64 = match reader.read_u32(byte_order) {
                Ok(r) => r.into(),
                Err(e) => return Err(e),
            };
            let next_entries_index_position = match reader.stream_position() {
                Ok(r) => r,
                Err(e) => return Err(e),
            };
            match reader.seek(std::io::SeekFrom::Start(header.body_start_position + offset)) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
            let mut entries = std::vec::Vec::<super::Label>::new();
            for _ in 0 .. count {
                let name_length: usize = match reader.read_u8() {
                    Ok(r) => r.into(),
                    Err(e) => return Err(e),
                };
                let mut name_bytes = vec![0; name_length];
                match reader.read_exact(&mut name_bytes) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                };
                let name = match std::string::String::from_utf8(name_bytes) {
                    Ok(r) => r,
                    Err(error) => return Err(
                        std::io::Error::new(std::io::ErrorKind::InvalidData, error)
                    ),
                };
                let index = match reader.read_u32(byte_order) {
                    Ok(r) => r as usize,
                    Err(e) => return Err(e),
                };
                entries.push(super::Label { name, index });
            }
            labels.push(entries);
            let _ = reader.seek(std::io::SeekFrom::Start(next_entries_index_position));
        }

        match reader.seek(std::io::SeekFrom::Start(header.body_start_position + u64::from(header.body_size))) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        match seek_section_end(reader) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        Ok(Self { labels })
    }
}

impl super::TextSection {
    pub(super) fn from_reader<Read: std::io::Read + std::io::Seek>(
        reader: &mut Read,
        byte_order: crate::bytes::ByteOrder,
        encoding: crate::Encoding,
    ) -> Result<Self, std::io::Error> {
        let header = match super::Header::from_reader(reader, byte_order) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        let mut offsets = std::vec::Vec::<u64>::new();
        for _ in 0 .. header.entries_count {
            offsets.push(match reader.read_u32(byte_order) {
                Ok(r) => r.into(),
                Err(e) => return Err(e),
            });
        }
        offsets.push(header.body_size.into());

        let mut texts = std::vec::Vec::<std::string::String>::new();
        for offset_index in 0..offsets.len() - 1 {
            let start = offsets[offset_index];
            let end = offsets[offset_index + 1];
            let text_length: usize = match (end - start).try_into() {
                Ok(r) => r,
                Err(error) => return Err(
                    std::io::Error::new(std::io::ErrorKind::InvalidData, error)
                ),
            };
            let mut text_bytes = vec![0; text_length];
            match reader.seek(std::io::SeekFrom::Start(header.body_start_position + start)) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
            match reader.read_exact(&mut text_bytes) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
            let text = match encoding {
                crate::Encoding::Utf8 => {
                    match std::string::String::from_utf8(text_bytes) {
                        Ok(r) => r,
                        Err(error) => return Err(
                            std::io::Error::new(std::io::ErrorKind::InvalidData, error)
                        ),
                    }
                }
                crate::Encoding::Utf16 => {
                    let (front, slice, back) = unsafe { text_bytes.align_to::<u16>() };
                    if front.is_empty() && back.is_empty() {
                        match std::string::String::from_utf16(slice) {
                            Ok(r) => r,
                            Err(error) => return Err(
                                std::io::Error::new(std::io::ErrorKind::InvalidData, error)
                            ),
                        }
                    } else {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Unable to align to UTF-16",
                        ));
                    }
                }
            };
            // Remove the null character
            texts.push(text.trim_end_matches(char::from(0)).into());
        }

        match reader.seek(std::io::SeekFrom::Start(header.body_start_position + u64::from(header.body_size))) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        match seek_section_end(reader) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        Ok(Self { texts })
    }
}

impl super::Header {
    pub(super) fn from_reader<Read: std::io::Read + std::io::Seek>(
        reader: &mut Read,
        byte_order: crate::bytes::ByteOrder
    ) -> Result<Self, std::io::Error> {
        let body_size = match reader.read_u32(byte_order) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        match reader.seek(std::io::SeekFrom::Current(8)) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        let body_start_position = match reader.stream_position() {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        let entries_count = match reader.read_u32(byte_order) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        Ok(Self { body_size, entries_count, body_start_position })
    }
}

fn seek_section_end<Read: std::io::Read + std::io::Seek>(reader: &mut Read,) -> Result<(), std::io::Error> {
    let current_position = match reader.stream_position() {
        Ok(r) => r,
        Err(e) => return Err(e),
    };
    let offset_to_end = 16 - (current_position % 16);
    if offset_to_end < 16 {
        match reader.seek(std::io::SeekFrom::Current(offset_to_end as i64)) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
    }
    Ok(())
}