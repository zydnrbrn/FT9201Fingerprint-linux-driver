use std::io;
use std::os::unix::io::{AsRawFd, RawFd};

pub struct FT9201Fingerprint {
    file_descriptor: RawFd,
}

pub fn main() {
    let mut fingerprint = FT9201Fingerprint::new("/dev/fingerprint").expect("Failed to open the device");
    let fingerprint_data = fingerprint.capture_fingerprint().expect("Failed to capture fingerprint data");
    println!("Fingerprint data: {:?}", fingerprint_data);
}

impl FT9201Fingerprint {
    pub fn new(device_path: &str) -> io::Result<Self> {
        let file_descriptor = open(device_path, libc::O_RDWR)?;
        Ok(FT9201Fingerprint {
            file_descriptor,
        })
    }

    pub fn capture_fingerprint(&mut self) -> io::Result<Vec<u8>> {
        let mut buffer = vec![0u8; 256];
        let bytes_read = read(self.file_descriptor, buffer.as_mut_slice())?;
        buffer.truncate(bytes_read);
        Ok(buffer)
    }
}

impl AsRawFd for FT9201Fingerprint {
    fn as_raw_fd(&self) -> RawFd {
        self.file_descriptor
    }
}

impl Drop for FT9201Fingerprint {
    fn drop(&mut self) {
        close(self.file_descriptor).expect("Failed to close the device");
    }
}

// Helper functions
fn open(path: &str, flags: i32) -> io::Result<RawFd> {
    let fd = unsafe { libc::open(path.as_ptr() as *const libc::c_char, flags) };
    if fd < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(fd)
    }
}

fn read(fd: RawFd, buf: &mut [u8]) -> io::Result<usize> {
    let result = unsafe { libc::read(fd, buf.as_mut_ptr() as *mut libc::c_void, buf.len()) };
    if result < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(result as usize)
    }
}

fn close(fd: RawFd) -> io::Result<()> {
    let result = unsafe { libc::close(fd) };
    if result < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}