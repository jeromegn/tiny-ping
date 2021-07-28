use std::net::IpAddr;

use crate::{EchoRequest as ER2, IcmpV4, IpV4Packet, ER1};

use crate::error::{Error, Result};

#[derive(Debug)]
pub struct EchoRequest {
    pub destination: IpAddr,
    pub ident: u16,
    pub seq_cnt: u16,
    pub size: usize,
}

impl EchoRequest {
    pub fn new(destination: IpAddr, ident: u16, seq_cnt: u16, size: usize) -> Self {
        EchoRequest {
            destination,
            ident,
            seq_cnt,
            size,
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>> {
        match self.destination {
            IpAddr::V4(_) => self.encode_icmp_v4(),
            IpAddr::V6(_) => self.encode_icmp_v6(),
        }
    }

    /// Encodes as an ICMPv4 EchoRequest.
    fn encode_icmp_v4(&self) -> Result<Vec<u8>> {
        let req = ER2 {
            ident: self.ident,
            seq_cnt: self.seq_cnt,
        };
        let mut buffer = vec![0; 8 + self.size]; // 8 bytes of header, then payload
        let payload = vec![0; self.size];
        req.encode::<IcmpV4>(&mut buffer, &payload)
    }

    /// Encodes as an ICMPv6 EchoRequest.
    fn encode_icmp_v6(&self) -> Result<Vec<u8>> {
        Err(Error::Unimplemented)
    }
}

/// `EchoReply` struct, which contains some packet information.
#[derive(Debug)]
pub struct EchoReply {
    /// IP Time To Live for outgoing packets. Present for ICMPv4 replies,
    /// absent for ICMPv6 replies.
    pub ttl: Option<u8>,
    /// Source address of ICMP packet.
    pub source: IpAddr,
    /// Sequence of ICMP packet.
    pub sequence: u16,
    /// Identifier of ICMP packet.
    pub identifier: u16,
    /// Size of ICMP packet.
    pub size: usize,
}

impl EchoReply {
    /// Unpack IP packets received from socket as `EchoReply` struct.
    pub fn decode(addr: IpAddr, buf: &[u8]) -> Result<EchoReply> {
        match addr {
            IpAddr::V4(_) => decode_icmpv4(addr, buf),
            IpAddr::V6(_) => decode_icmpv6(addr, buf),
        }
    }
}

/// Decodes an ICMPv4 packet received from an IPv4 raw socket
fn decode_icmpv4(addr: IpAddr, buf: &[u8]) -> Result<EchoReply> {
    let ipv4_decoded = IpV4Packet::decode(buf)?;
    let icmp_decoded = ER1::decode::<IcmpV4>(ipv4_decoded.data)?;
    Ok(EchoReply {
        ttl: None,
        source: addr,
        sequence: icmp_decoded.seq_cnt,
        identifier: icmp_decoded.ident,
        size: icmp_decoded.payload.len(),
    })
}

/// Decodes an ICMPv6 packet received from an IPv6 raw socket
fn decode_icmpv6(_addr: IpAddr, _buf: &[u8]) -> Result<EchoReply> {
    Err(Error::Unimplemented)
}
