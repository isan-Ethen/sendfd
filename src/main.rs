use libc::{bind, connect, socket};
use std::ffi::CString;
use std::io::{self};
use std::mem;

type Result<T> = std::result::Result<T, io::Error>;

fn from_syscall_error(error: syscall::Error) -> io::Error {
    io::Error::from_raw_os_error(error.errno as i32)
}

fn str2c_char_array(s: String) -> Result<[c_char; 108], ()> {
    if s.len() > 108 {
        eprintln!("path is longer than 108");
        return Err(());
    }
    match CString::new(s) {
        Ok(c_string) => {
            let bytes = c_string.as_bytes_with_nul();

            let mut array = [0 as c_char; 108];
            for (i, &byte) in bytes.iter().enumerate().take(108) {
                array[i] = byte as c_char;
            }
            Ok(array)
        }
        Err(_) => Err(()),
    }
}

fn connect_gate(path: &str) -> Result<RawFd> {
    // make socket
    let gate = unsafe { socket(libc::AF_UNIX, libc::SOCK_DGRAM, 0) };
    if gate < 0 {
        return Err(());
    }

    let c_path = CString::new(path)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "path contains null bytes"))?;

    // initialize socket addr
    let mut gate_addr: libc::sockaddr_un = unsafe { mem::zeroed() };
    gate_addr.sun_family = libc::AF_UNIX as libc::sa_family_t;

    // convert cstring to c_char array
    unsafe {
        let path_bytes = c_path.as_bytes_with_nul();
        let dest_len = gate_addr.sun_path.len();

        if path_bytes.len() > gate_addr.sun_path.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "path is too long",
            ));
        }

        for (i, &byte) in path_bytes.iter().enumerate() {
            gate_addr.sun_path[i] = byte as libc::c_char;
        }
    }

    // connect socket
    if connect(
        gate,
        &gate_addr as *const _ as *const libc::sockaddr,
        mem::size_of::<libc::sockaddr_un>() as libc::socklen_t,
    ) < 0
    {
        let err = io::Error::last_os_error();
        unsafe { libc::close(gate) };
        return Err(err);
    }

    Ok(gate)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fd_path = format!("chan:{}", "/tmp/unix-domain-socket/test");
    println!("scheme path: {}", fd_path);

    println!("open sender");
    let sender_fd = connect_gate(&fd_path)?;

    let path = "file:/home/user/test.txt";
    println!("file open: {}", path);
    let socket_fd = syscall::open(path, syscall::O_RDWR).map_err(from_syscall_error)?;

    println!("sendfd");
    let res = syscall::sendfd(sender_fd, socket_fd, 0, 0).map_err(from_syscall_error)?;

    println!("res: {}", res);

    Ok(())
}
