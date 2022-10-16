use tap::TapDevice;

use crate::{ethernet::{EthernetFrame, EtherType}, arp::ArpPacket};

mod tap;
mod ethernet;
mod arp;


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
                let arp = ArpPacket::from_bytes(frame.payload());
                println!("{}", arp);
            }
        }
    }
}
