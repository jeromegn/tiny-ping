#[derive(Debug)]
pub enum Error {
    InvalidProtocol,
    Internal,
    TooSmallHeader,
    InvalidHeaderSize,
    InvalidVersion,
    UnknownProtocol,
    InvalidSize,
    InvalidPacket,
    Io(String),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.to_string())
    }
}
