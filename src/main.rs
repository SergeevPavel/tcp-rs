use tap::TapDevice;

mod tap;


fn main() {
    TapDevice::with_name(&"").unwrap();
}
