use std::env;
use std::fs::File;
use std::io;
use std::os::unix::io::AsRawFd;

fn from_syscall_error(error: syscall::Error) -> io::Error {
    io::Error::from_raw_os_error(error.errno as i32)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    println!("file path: {}", path);
    let file = File::open(path)?;
    let fd = File::as_raw_fd(&file) as usize;

    let fd_path = format!("chan:{}", "/tmp/unix-domain-socket/test");

    println!("open sender");
    let sender_fd =
        syscall::open(fd_path, syscall::O_CREAT | syscall::O_RDWR).map_err(from_syscall_error)?;

    println!("sendfd");
    let res = syscall::sendfd(sender_fd, fd, 0, 0).map_err(from_syscall_error)?;

    println!("res: {}", res);

    Ok(())
}
