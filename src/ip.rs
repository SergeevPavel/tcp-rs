use std::fmt::Display;


pub struct IpAddress(pub [u8; 4]);

impl IpAddress {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() == 4);
        let mut buffer = [0u8; 4];
        buffer.copy_from_slice(bytes);
        IpAddress(buffer)
    }
}

impl Display for IpAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}:{}:{}:{}", self.0[0], self.0[1], self.0[2], self.0[3]))
    }
}