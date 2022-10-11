use std::io;

pub const SIOCGIFMTU: libc::c_ulong = 0x8921;
pub const SIOCGIFINDEX: libc::c_ulong = 0x8933;
pub const ETH_P_ALL: libc::c_short = 0x0003;
pub const ETH_P_IEEE802154: libc::c_short = 0x00F6;

pub const TUNSETIFF: libc::c_ulong = 0x400454CA;
pub const IFF_TUN: libc::c_int = 0x0001;
pub const IFF_TAP: libc::c_int = 0x0002;
pub const IFF_NO_PI: libc::c_int = 0x1000;

#[repr(C)]
struct ifreq {
    ifr_name: [libc::c_char; libc::IF_NAMESIZE],
    ifr_data: libc::c_int, /* ifr_ifindex or ifr_mtu */
}

fn ifreq_for(name: &str, flags: libc::c_int) -> ifreq {
    let mut ifreq = ifreq {
        ifr_name: [0; libc::IF_NAMESIZE],
        ifr_data: flags,
    };
    for (i, byte) in name.as_bytes().iter().enumerate() {
        ifreq.ifr_name[i] = *byte as libc::c_char
    }
    ifreq
}

fn ifreq_ioctl(
        lower: libc::c_int,
        ifreq: &mut ifreq,
        cmd: libc::c_ulong,
        ) -> io::Result<libc::c_int> {
    unsafe {
        let res = libc::ioctl(lower, cmd as _, ifreq as *mut ifreq);
        if res == -1 {
            return Err(io::Error::last_os_error());
        }
    }

    Ok(ifreq.ifr_data)
}

pub struct TapDevice {
    fd: libc::c_int,
}

impl TapDevice {
    pub fn with_name(name: &str) -> io::Result<Self> {
        let fd = unsafe {
            let fd = libc::open("/dev/net/tun\0".as_ptr() as *const libc::c_char, libc::O_RDWR | libc::O_NONBLOCK);
            if fd == -1 {
                return Err(std::io::Error::last_os_error());
            }
            fd
        };
        let mut ifreq = ifreq_for(name, IFF_TAP | IFF_NO_PI);
        ifreq_ioctl(fd, &mut ifreq, TUNSETIFF)?;
        return Ok(TapDevice {
            fd
        });
    }
}