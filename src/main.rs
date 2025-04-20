use libc::{connect, socket, write};
use std::ffi::CString;
use std::io::{self};
use std::mem;
use std::os::unix::io::RawFd;

type Result<T> = std::result::Result<T, io::Error>;

fn from_syscall_error(error: syscall::Error) -> io::Error {
    io::Error::from_raw_os_error(error.errno as i32)
}

fn main() -> Result<()> {
    let path = "/scheme/file/home/user/test";
    println!("file open: {}", path);
    let fd = syscall::open(path, syscall::O_RDWR).map_err(from_syscall_error)?;

    println!("fd path");
    let fd_path = "/tmp/uds/test";
    let scheme_path = format!("/scheme/chan{}", fd_path);
    println!("scheme path: {}", scheme_path);

    println!("connect gate");
    let gate = syscall::open(scheme_path, syscall::O_RDWR).map_err(from_syscall_error)?;

    println!("sendfd");
    let res = syscall::sendfd(gate, fd, 0, 0).map_err(from_syscall_error)?;

    let message = "hello";
    let res = unsafe {
        write(
            gate,
            message.as_ptr() as *const std::os::raw::c_void,
            message.len(),
        )
    };
    println!("res: {}", res);

    syscall::close(gate).map_err(from_syscall_error)?;

    Ok(())
}
