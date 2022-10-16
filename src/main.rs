use tap::TapDevice;

use crate::{ethernet::{EthernetFrame, EtherType}, arp::{ArpHeader, HardwareType, ArpPayload}};

mod tap;
mod ethernet;
mod arp;
mod ip;


fn main() {
    let name = std::env::args().nth(1).unwrap();
    let mut device = TapDevice::with_name(&name).unwrap();
    device.set_address("10.0.0.5");
    device.interface_up();
    device.wait(None).unwrap();
    loop {
        if let Ok(frame) = device.recv_frame() {
            let frame = EthernetFrame::new(frame);
            println!("{}", frame);
            if frame.ethertype() == Some(EtherType::Arp) {
                let arp_hdr = ArpHeader::from_bytes(frame.payload());
                println!("{}", arp_hdr);
                if let Some(arp_payload) = ArpPayload::from_arp_hdr(arp_hdr) {
                    println!("{}", arp_payload);
                }
            }
        }
    }
}
