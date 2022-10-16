use std::{ops::{Range, RangeFrom}, fmt::Display};

use byteorder::{NetworkEndian, ByteOrder};

use crate::{ethernet::EtherType, ip::IpAddress};
use crate::ethernet::MacAddress;

pub struct ArpHeader<'a> {
    buffer: &'a[u8]
}

impl <'a> Display for ArpHeader<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[ARP_HDR: {:?} {:?} {:?}]", self.hardware_type(), self.protocol_type(), self.operation()))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum HardwareType {
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
    Reply,
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

#[allow(dead_code)]
impl <'a> ArpHeader<'a> {
    const HARDWARE_TYPE: Range<usize> = 0..2;
    const PROTOCOL_TYPE: Range<usize> = 2..4;
    const HARDWARE_ADDR_SIZE: usize = 5;
    const PROTOCOL_ADDR_SIZE: usize = 6;
    const OPERATION: Range<usize> = 6..8;
    const PAYLOAD: RangeFrom<usize> = 8..;

    pub fn from_bytes(buffer: &'a[u8]) -> Self {
        ArpHeader { buffer }
    }

    pub fn hardware_type(&self) -> Option<HardwareType> {
        let code = NetworkEndian::read_u16(&self.buffer[ArpHeader::HARDWARE_TYPE]);
        HardwareType::decode(code)
    }

    pub fn protocol_type(&self) -> Option<EtherType> {
        let code = NetworkEndian::read_u16(&self.buffer[ArpHeader::PROTOCOL_TYPE]);
        EtherType::decode(code)
    }

    pub fn operation(&self) -> Option<Operation> {
        let code = NetworkEndian::read_u16(&self.buffer[ArpHeader::OPERATION]);
        Operation::decode(code)
    }

    pub fn payload(&self) -> &'a[u8] {
        &self.buffer[ArpHeader::PAYLOAD]
    }
}

pub struct ArpPayload<'a> {
    buffer: &'a[u8]
}

impl <'a> ArpPayload<'a> {
    const SOURCE_MAC: Range<usize> = 0..6;
    const SOURCE_IP: Range<usize> = 6..10;
    const DEST_MAC: Range<usize> = 10..16;
    const DEST_IP: Range<usize> = 16..20;

    pub fn from_bytes<'b>(buffer: &'b[u8]) -> ArpPayload<'b> {
        ArpPayload { buffer }
    }

    pub fn from_arp_hdr<'b>(arp_hdr: ArpHeader<'b>) -> Option<ArpPayload<'b>> {
        if arp_hdr.hardware_type() == Some(HardwareType::Ethernet) && arp_hdr.protocol_type() == Some(EtherType::Ipv4) {
            Some(Self::from_bytes(arp_hdr.payload()))
        } else {
            None
        }
    }

    pub fn source_mac(&self) -> MacAddress {
        MacAddress::from_bytes(&self.buffer[ArpPayload::SOURCE_MAC])
    }

    pub fn source_ip(&self) -> IpAddress {
        IpAddress::from_bytes(&self.buffer[ArpPayload::SOURCE_IP])
    }

    pub fn dest_mac(&self) -> MacAddress {
        MacAddress::from_bytes(&self.buffer[ArpPayload::DEST_MAC])
    }

    pub fn dest_ip(&self) -> IpAddress {
        IpAddress::from_bytes(&self.buffer[ArpPayload::DEST_IP])
    }
}

impl <'a> Display for ArpPayload<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[ARP_PL {} {} {} {}]", self.source_mac(), self.source_ip(), self.dest_mac(), self.dest_ip()))
    }
}