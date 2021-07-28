pub type Result<T> = std::result::Result<T, Error>;

/// An error resulting from a ping option-setting or send/receive operation.
#[derive(Debug)]
pub enum Error {
    IncorrectBufferSize,
    NotIpv4Packet,
    NotIcmpPacket,
    NotIcmpv6Packet,
    PayloadTooShort { got: usize, want: usize },
    IOError(String),
    NotEchoReply,
    NotV6EchoReply,
    Timeout,
    OtherICMP,
    InvalidSize,
    InvalidPacket,
    TooSmallHeader,
    InvalidHeaderSize,
    InvalidVersion,
    UnknownProtocol,
    Unimplemented,
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl std::error::Error for Error {}
