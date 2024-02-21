pub(crate) mod read;

#[derive(Clone, Copy)]
pub(crate) enum ByteOrder {
    BigEndian,
    LittleEndian,
}
