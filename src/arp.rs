use std::{ops::Range, fmt::Display};

use byteorder::{NetworkEndian, ByteOrder};

use crate::ethernet::EtherType;

pub struct ArpPacket<'a> {
    buffer: &'a[u8]
}

impl <'a> Display for ArpPacket<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[ARP: {:?} {:?} {:?}]", self.hardware_type(), self.protocol_type(), self.operation()))
    }
}

#[derive(Debug, Eq, PartialEq)]
enum HardwareType {
    Ethernet
}

impl HardwareType {
    const ARP_ETHERNET: u16 = 0x0001;

    fn decode(code: u16) -> Option<Self> {
        match code {
            HardwareType::ARP_ETHERNET => Some(HardwareType::Ethernet),
            _ => None
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Operation {
    Request,
    Reply
}

impl Operation {
    const ARP_REQUEST: u16 = 0x0001;
    const ARP_REPLY: u16 = 0x0002;

    fn decode(code: u16) -> Option<Self> {
        match code {
            Operation::ARP_REQUEST => Some(Operation::Request),
            Operation::ARP_REPLY => Some(Operation::Reply),
            _ => None
        }
    }
}

impl <'a> ArpPacket<'a> {
    const HARDWARE_TYPE: Range<usize> = 0..2;
    const PROTOCOL_TYPE: Range<usize> = 2..4;
    const HARDWARE_ADDR_SIZE: usize = 5;
    const PROTOCOL_ADDR_SIZE: usize = 6;
    const OPERATION: Range<usize> = 6..8;

    pub fn from_bytes(buffer: &'a[u8]) -> Self {
        ArpPacket { buffer }
    }

    pub fn hardware_type(&self) -> Option<HardwareType> {
        let code = NetworkEndian::read_u16(&self.buffer[ArpPacket::HARDWARE_TYPE]);
        HardwareType::decode(code)
    }

    pub fn protocol_type(&self) -> Option<EtherType> {
        let code = NetworkEndian::read_u16(&self.buffer[ArpPacket::PROTOCOL_TYPE]);
        EtherType::decode(code)
    }

    pub fn operation(&self) -> Option<Operation> {
        let code = NetworkEndian::read_u16(&self.buffer[ArpPacket::OPERATION]);
        Operation::decode(code)
    }
}