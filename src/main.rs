use libc::{connect, socket};
use std::ffi::CString;
use std::io::{self};
use std::mem;
use std::os::unix::io::RawFd;

type Result<T> = std::result::Result<T, io::Error>;

fn from_syscall_error(error: syscall::Error) -> io::Error {
    io::Error::from_raw_os_error(error.errno as i32)
}

fn connect_gate(path: &str) -> Result<RawFd> {
    // make socket
    let gate = unsafe { socket(libc::AF_UNIX, libc::SOCK_DGRAM, 0) };
    if gate < 0 {
        return Err(io::Error::last_os_error());
    }

    let c_path = CString::new(path)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "path contains null bytes"))?;

    // initialize socket addr
    let mut gate_addr: libc::sockaddr_un = unsafe { mem::zeroed() };
    gate_addr.sun_family = libc::AF_UNIX as libc::sa_family_t;

    let path_bytes = c_path.as_bytes_with_nul();

    // check len of path
    if path_bytes.len() > gate_addr.sun_path.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "path is too long",
        ));
    }

    // write path to gate_addr
    for (i, &byte) in path_bytes.iter().enumerate() {
        gate_addr.sun_path[i] = byte as libc::c_char;
    }

    // connect socket
    let connect_result = unsafe {
        connect(
            gate,
            &gate_addr as *const _ as *const libc::sockaddr,
            mem::size_of::<libc::sockaddr_un>() as libc::socklen_t,
        )
    };
    if connect_result < 0 {
        let err = io::Error::last_os_error();
        unsafe { libc::close(gate) };
        return Err(err);
    }

    Ok(gate)
}

fn main() -> Result<()> {
    let fd_path = format!("chan:{}", "/tmp/unix-domain-socket/test");
    println!("scheme path: {}", fd_path);

    println!("open sender");
    let sender_fd = connect_gate(&fd_path)?;

    let path = "file:/home/user/test.txt";
    println!("file open: {}", path);
    let socket_fd = syscall::open(path, syscall::O_RDWR).map_err(from_syscall_error)?;

    println!("sendfd");
    let res = syscall::sendfd(
        sender_fd
            .try_into()
            .map_err(|_| io::Error::last_os_error())?,
        socket_fd,
        0,
        0,
    )
    .map_err(from_syscall_error)?;

    println!("res: {}", res);

    Ok(())
}
