use std::{io, time::Duration, mem, ptr};

#[allow(dead_code)]
mod constants {
    pub const SIOCGIFMTU: libc::c_ulong = 0x8921;
    pub const SIOCGIFINDEX: libc::c_ulong = 0x8933;
    pub const ETH_P_ALL: libc::c_short = 0x0003;
    pub const ETH_P_IEEE802154: libc::c_short = 0x00F6;

    pub const TUNSETIFF: libc::c_ulong = 0x400454CA;
    pub const IFF_TUN: libc::c_int = 0x0001;
    pub const IFF_TAP: libc::c_int = 0x0002;
    pub const IFF_NO_PI: libc::c_int = 0x1000;
}


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
    name: String,
    fd: libc::c_int,
}

impl TapDevice {
    const MTU: usize = 1500;
    const ETH_HDR_SIZE: usize = 100;

    pub fn with_name(name: &str) -> io::Result<Self> {
        let fd = unsafe {
            let fd = libc::open("/dev/net/tun\0".as_ptr() as *const libc::c_char, libc::O_RDWR | libc::O_NONBLOCK);
            if fd == -1 {
                return Err(std::io::Error::last_os_error());
            }
            fd
        };
        let mut ifreq = ifreq_for(name, constants::IFF_TAP | constants::IFF_NO_PI);
        ifreq_ioctl(fd, &mut ifreq, constants::TUNSETIFF)?;
        return Ok(TapDevice {
            fd,
            name: name.to_owned()
        });
    }

    pub fn recv_frame(&mut self) -> io::Result<Vec<u8>> {
        let mut buffer = vec![0u8; TapDevice::MTU + TapDevice::ETH_HDR_SIZE];
        let ptr = buffer.as_mut_ptr();
        let read_bytes = unsafe {
            let result = libc::read(self.fd, ptr as *mut libc::c_void, buffer.len());
            if result < 0 {
                return Err(std::io::Error::last_os_error());
            }
            result as usize
        };
        buffer.resize(read_bytes, 0u8);
        return Ok(buffer)
    }

    /// Wait until given file descriptor becomes readable, but no longer than given timeout.
    pub fn wait(&mut self, duration: Option<Duration>) -> io::Result<()> {
        unsafe {
            let mut readfds = {
                let mut readfds = mem::MaybeUninit::<libc::fd_set>::uninit();
                libc::FD_ZERO(readfds.as_mut_ptr());
                libc::FD_SET(self.fd, readfds.as_mut_ptr());
                readfds.assume_init()
            };

            let mut writefds = {
                let mut writefds = mem::MaybeUninit::<libc::fd_set>::uninit();
                libc::FD_ZERO(writefds.as_mut_ptr());
                writefds.assume_init()
            };

            let mut exceptfds = {
                let mut exceptfds = mem::MaybeUninit::<libc::fd_set>::uninit();
                libc::FD_ZERO(exceptfds.as_mut_ptr());
                exceptfds.assume_init()
            };

            let mut timeout = libc::timeval {
                tv_sec: 0,
                tv_usec: 0,
            };
            let timeout_ptr = if let Some(duration) = duration {
                timeout.tv_sec = duration.as_secs() as libc::time_t;
                timeout.tv_usec = (duration.subsec_millis() * 1_000) as libc::suseconds_t;
                &mut timeout as *mut _
            } else {
                ptr::null_mut()
            };

            let res = libc::select(
                    self.fd + 1,
                    &mut readfds,
                    &mut writefds,
                    &mut exceptfds,
                    timeout_ptr,
            );
            if res == -1 {
                return Err(io::Error::last_os_error());
            }
            Ok(())
        }
    }

    pub fn set_address(&mut self, address: &str) {
        std::process::Command::new("ip")
            .args(["address", "add", "dev", &self.name, address])
            .spawn().unwrap().wait().unwrap();
    }

    pub fn interface_up(&mut self) {
        std::process::Command::new("ip")
            .args(["link", "set", "dev", &self.name, "up"])
            .spawn().unwrap().wait().unwrap();
    }
}

impl Drop for TapDevice {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.fd);
        }
    }
}
