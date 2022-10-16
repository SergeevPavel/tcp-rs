use tap::TapDevice;

use crate::ethernet::EthernetFrame;

mod tap;
mod ethernet;


fn main() {
    let name = std::env::args().nth(1).unwrap();
    let mut device = TapDevice::with_name(&name).unwrap();
    device.set_address("10.0.0.5");
    device.interface_up();
    device.wait(None).unwrap();
    loop {
        if let Ok(frame) = device.recv_frame() {
            let frame = EthernetFrame::new(frame);
            println!("From: {} To: {} Type: {:?} Payload len: {}",
                     frame.source(),
                     frame.destination(),
                     frame.ethertype(),
                     frame.payload().len());
        }
    }
}
