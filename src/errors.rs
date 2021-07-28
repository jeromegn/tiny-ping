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
    Io,
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Self::Io
    }
}
