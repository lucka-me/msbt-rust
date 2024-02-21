mod read;

pub enum Section {
    Attribute(AttributeSection),
    Label(LabelSection),
    Text(TextSection),
}

pub struct AttributeSection {
}

pub struct Label {
    pub name: std::string::String,
    pub index: usize,
}

pub struct LabelSection {
    pub labels: std::vec::Vec<std::vec::Vec<Label>>,
}

pub struct TextSection {
    pub texts: std::vec::Vec<std::string::String>,
}

pub(crate) fn from_reader<Read: std::io::Read + std::io::Seek>(
    reader: &mut Read,
    byte_order: crate::bytes::ByteOrder,
    encoding: crate::Encoding,
) -> Result<Section, std::io::Error> {
    let mut section_identifier = [0; consts::SECTION_IDENTIFIER_LENGTH];
    match reader.read_exact(&mut section_identifier) {
        Ok(_) => (),
        Err(e) => return Err(e),
    };
    match section_identifier {
        consts::ATTRIBUTE_SECTION_IDENTIFIER => {
            let section = match AttributeSection::from_reader(reader, byte_order) {
                Ok(r) => r,
                Err(e) => return Err(e),
            };
            Ok(Section::Attribute(section))
        }
        consts::LABEL_SECTION_IDENTIFIER => {
            let section = match LabelSection::from_reader(reader, byte_order) {
                Ok(r) => r,
                Err(e) => return Err(e),
            };
            Ok(Section::Label(section))
        }
        consts::TEXT_SECTION_IDENTIFIER => {
            let section = match TextSection::from_reader(reader, byte_order, encoding) {
                Ok(r) => r,
                Err(e) => return Err(e),
            };
            Ok(Section::Text(section))
        }
        _ => {
            Err(std::io::Error::new(std::io::ErrorKind::Unsupported, "Unknown section identifier"))
        }
    }
}

mod consts {
    pub const SECTION_IDENTIFIER_LENGTH: usize = 4;
    pub const ATTRIBUTE_SECTION_IDENTIFIER: [u8; SECTION_IDENTIFIER_LENGTH] = *b"ATR1";
    pub const LABEL_SECTION_IDENTIFIER: [u8; SECTION_IDENTIFIER_LENGTH] = *b"LBL1";
    pub const TEXT_SECTION_IDENTIFIER: [u8; SECTION_IDENTIFIER_LENGTH] = *b"TXT2";
}

struct Header {
    pub body_size: u32,
    pub entries_count: u32,
    pub body_start_position: u64,
}