use std::{ops::{Range, RangeFrom}, fmt::Display};

use byteorder::{NetworkEndian, ByteOrder};


pub struct MacAdress(pub [u8; 6]);

impl MacAdress {
    fn from_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() == 6);
        let mut buffer = [0u8; 6];
        buffer.copy_from_slice(bytes);
        MacAdress(buffer)
    }
}

impl Display for MacAdress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                                 self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]))
    }
}

#[derive(Debug)]
pub enum EtherType {
    Ipv4,
    Arp,
    Ipv6
}

impl EtherType {
    const ETH_P_ARP: u16 = 0x0806;
    const ETH_P_IP: u16 = 0x0800;
    const ETH_P_IPV6: u16 = 0x86DD;

    fn decode(v: u16) -> Option<Self> {
        match v {
            EtherType::ETH_P_ARP => Some(EtherType::Arp),
            EtherType::ETH_P_IP => Some(EtherType::Ipv4),
            EtherType::ETH_P_IPV6 => Some(EtherType::Ipv6),
            _ => None
        }
    }
}

pub struct EthernetFrame {
    buffer: Vec<u8>
}

impl EthernetFrame {
    const DEST: Range<usize> = 0..6;
    const SRC: Range<usize> = 6..12;
    const TYPE: Range<usize> = 12..14;
    const EHR_HDR_SIZE: usize = 14;
    const PAYLOAD: RangeFrom<usize> = 14..;

    pub fn new (buffer: Vec<u8>) -> Self {
        assert!(buffer.len() >= EthernetFrame::EHR_HDR_SIZE);
        EthernetFrame { buffer }
    }

    pub fn destination(&self) -> MacAdress {
        MacAdress::from_bytes(&self.buffer[EthernetFrame::DEST])
    }

    pub fn source(&self) -> MacAdress {
        MacAdress::from_bytes(&self.buffer[EthernetFrame::SRC])
    }

    pub fn ethertype(&self) -> Option<EtherType> {
        let code = NetworkEndian::read_u16(&self.buffer[EthernetFrame::TYPE]);
        EtherType::decode(code)
    }

    pub fn payload(&self) -> &[u8] {
        return &self.buffer[EthernetFrame::PAYLOAD]
    }
}