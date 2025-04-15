use std::fs::File;
use std::io;
use std::os::unix::io::AsRawFd;

fn from_syscall_error(error: syscall::Error) -> io::Error {
    io::Error::from_raw_os_error(error.errno as i32)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fd_path = format!("/scheme/chan/{}", "tmp/unix-domain-socket/test");
    println!("scheme path: {}", fd_path);

    println!("open sender");
    let sender_fd =
        syscall::open(fd_path, syscall::O_CREAT | syscall::O_RDWR).map_err(from_syscall_error)?;

    println!("file open");
    let file = File::open("/home/user/test")?;
    let socket_fd = file.as_raw_fd() as usize;
    println!("sendfd");
    let res = syscall::sendfd(sender_fd, socket_fd, 0, 0).map_err(from_syscall_error)?;

    println!("res: {}", res);

    Ok(())
}
