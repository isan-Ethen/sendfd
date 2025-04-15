use std::io;

fn from_syscall_error(error: syscall::Error) -> io::Error {
    io::Error::from_raw_os_error(error.errno as i32)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fd_path = format!("chan:{}", "/tmp/unix-domain-socket/test");
    println!("scheme path: {}", fd_path);

    println!("file open");
    let socket_fd = syscall::open("/tmp/test.txt", syscall::O_CREAT | syscall::O_RDWR)
        .map_err(from_syscall_error)?;

    println!("open sender");
    let sender_fd =
        syscall::open(fd_path, syscall::O_CREAT | syscall::O_RDWR).map_err(from_syscall_error)?;

    println!("sendfd");
    let res = syscall::sendfd(sender_fd, socket_fd, 0, 0).map_err(from_syscall_error)?;

    println!("res: {}", res);

    Ok(())
}
