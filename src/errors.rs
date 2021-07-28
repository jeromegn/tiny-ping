#[derive(Debug)]
pub enum Error {
    InvalidProtocol,
    InternalError,
    TooSmallHeader,
    InvalidHeaderSize,
    InvalidVersion,
    UnknownProtocol,
    InvalidSize,
    InvalidPacket,
    IoError,
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Self::IoError
    }
}
