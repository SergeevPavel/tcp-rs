use tap::TapDevice;

mod tap;


fn main() {
    TapDevice::with_name(&"").unwrap();
    loop {
        std::thread::sleep_ms(1000);
    }
}
