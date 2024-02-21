mod bytes;
mod header;
mod section;

pub struct Message {
    pub attribute_section: Option<section::AttributeSection>,
    pub label_section: Option<section::LabelSection>,
    pub text_section: Option<section::TextSection>,
}

impl Message {
    pub fn from_file(file: std::fs::File) -> Result<Self, std::io::Error> {
        let file_size = match file.metadata() {
            Ok(r) => r.len(),
            Err(e) => return Err(e),
        };

        let mut reader = std::io::BufReader::new(&file);

        Self::from_reader(& mut reader, Some(file_size))
    }

    pub fn from_reader<Read: std::io::Read + std::io::Seek>(
        reader: &mut Read,
        total_size_to_check: Option<u64>
    ) -> Result<Self, std::io::Error> {
        if let Some(total_size) = total_size_to_check {
            // Check if file is aligned to 16 bytes
            if total_size % 16 != 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("The input ({total_size} bytes) is not aligned to 16 bytes."),
                ));
            }
        }
        let header = match header::MessageHeader::from_reader(reader) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        if let Some(total_size) = total_size_to_check {
            if total_size != header.file_size.into() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!(
                        "Expect {} bytes but actual {} bytes.",
                        header.file_size, total_size
                    ),
                ));
            }
        }

        let mut message = Self {
            attribute_section: None,
            label_section: None,
            text_section: None,
        };
        for _ in 0..header.sections_count {
            match section::from_reader(reader, header.byte_order, header.encoding) {
                Ok(r) => match r {
                    section::Section::Attribute(content) => {
                        message.attribute_section = Some(content)
                    }
                    section::Section::Label(content) => message.label_section = Some(content),
                    section::Section::Text(content) => message.text_section = Some(content),
                },
                Err(e) => return Err(e),
            };
        }
        Ok(message)
    }
}

#[derive(Clone, Copy)]
enum Encoding {
    Utf8,
    Utf16,
}
