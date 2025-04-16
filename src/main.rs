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
    println!("make socket");
    let gate = unsafe { socket(libc::AF_UNIX, libc::SOCK_STREAM, 0) };
    if gate < 0 {
        return Err(io::Error::last_os_error());
    }

    let c_path = CString::new(path)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "path contains null bytes"))?;

    println!("initialize socket addr");
    let mut gate_addr: libc::sockaddr_un = unsafe { mem::zeroed() };
    println!("set sun_family");
    let family_value = libc::AF_UNIX;
    println!("family_value: {}", family_value);
    let sa_family = family_value as libc::sa_family_t;
    println!("sa_family: {}", sa_family);
    gate_addr.sun_family = sa_family;
    println!("get path bytes");
    let path_bytes = c_path.as_bytes_with_nul();
    println!("path bytes len: {}", path_bytes.len());

    println!("check len of path");
    if path_bytes.len() > gate_addr.sun_path.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "path is too long",
        ));
    }

    println!("write path to gate_addr");
    for (i, &byte) in path_bytes.iter().enumerate() {
        gate_addr.sun_path[i] = byte as libc::c_char;
    }

    println!("connect socket");
    let connect_result = unsafe {
        connect(
            gate,
            &gate_addr as *const _ as *const libc::sockaddr,
            mem::size_of::<libc::sockaddr_un>() as libc::socklen_t,
        )
    };
    println!("connect result");
    if connect_result < 0 {
        let err = io::Error::last_os_error();
        unsafe { libc::close(gate) };
        return Err(err);
    }

    Ok(gate)
}

fn main() -> Result<()> {
    let path = "file:/home/user/test.txt";
    println!("file open: {}", path);
    let fd = syscall::open(path, syscall::O_RDWR).map_err(from_syscall_error)?;

    let fd_path = "/tmp/uds/test";
    let scheme_path = format!("chan:{}", fd_path);
    println!("scheme path: {}", scheme_path);

    println!("connect gate");
    // let sender_fd = connect_gate(&fd_path)?;
    let sender_fd = syscall::open(scheme_path, syscall::O_RDWR)?;

    println!("sendfd");
    // let res = syscall::sendfd(
    //     sender_fd
    //         .try_into()
    //         .map_err(|_| io::Error::last_os_error())?,
    //     fd,
    //     0,
    //     0,
    // )
    // .map_err(from_syscall_error)?;
    let res =
        syscall::sendfd(sender_fd, fd, 0, "recvfd".as_ptr() as u64).map_err(from_syscall_error)?;

    println!("res: {}", res);

    Ok(())
}
