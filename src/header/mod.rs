mod read;

pub(crate) struct MessageHeader {
    pub byte_order: crate::bytes::ByteOrder,
    pub encoding: crate::Encoding,
    pub sections_count: u16,
    pub file_size: u32,
}

mod consts {
    pub const MESSAGE_HEADER_IDENTIFIER_LENGTH: usize = 8;
    pub const MESSAGE_HEADER_IDENTIFIER: [u8; MESSAGE_HEADER_IDENTIFIER_LENGTH] = *b"MsgStdBn";
    pub const LITTLE_ENDIAN_IDENTIFIER: u16 = 0xFFFE;
    pub const BIG_ENDIAN_IDENTIFIER: u16 = 0xFEFF;
}
