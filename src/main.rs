//use libc::{
//    bind, connect,
//    header::{
//        arpa_inet::inet_aton,
//        errno::{EAFNOSUPPORT, EDOM, EFAULT, EINVAL, ENOSYS, EOPNOTSUPP, EPROTONOSUPPORT},
//        netinet_in::{in_addr, in_port_t, sockaddr_in},
//        string::strnlen,
//        sys_socket::{constants::*, msghdr, sa_family_t, sockaddr, socklen_t},
//        sys_time::timeval,
//        sys_un::sockaddr_un,
//    },
//    recvmsg, sendmsg, socket,
//};
use std::ffi::CString;
use std::io::{self};
use std::mem;
use std::os::raw::c_char;

fn from_syscall_error(error: syscall::Error) -> io::Error {
    io::Error::from_raw_os_error(error.errno as i32)
}

// fn str2c_char_array(s: String) -> Result<[c_char; 108], ()> {
//     if s.len() > 108 {
//         eprintln!("path is longer than 108");
//         return Err(());
//     }
//     match CString::new(s) {
//         Ok(c_string) => {
//             let bytes = c_string.as_bytes_with_nul();
//
//             let mut array = [0 as c_char; 108];
//             for (i, &byte) in bytes.iter().enumerate().take(108) {
//                 array[i] = byte as c_char;
//             }
//             Ok(array)
//         }
//         Err(_) => Err(()),
//     }
// }
//
// unsafe fn connect_gate(path: String) -> Result<usize, ()> {
//     let gate = socket(AF_UNIX, SOCK_DGRAM, 0);
//     if gate < 0 {
//         return Err(());
//     }
//
//     let sun_path: [c_char; 108] = str2c_char_array(path)?;
//
//     let gate_addr: sockaddr_un = sockaddr_un {
//         sun_family: syscall::AF_UNIX,
//         sun_path,
//     };
//
//     if connect(gate, gate_addr, mem::size_of_val(gate_addr)) < 0 {
//         return Err(());
//     }
//
//     Ok(gate)
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fd_path = format!("chan:{}", "/tmp/unix-domain-socket/test");
    println!("scheme path: {}", fd_path);

    println!("open sender");
    let sender_fd =
        syscall::open(fd_path, syscall::O_CREAT | syscall::O_RDWR).map_err(from_syscall_error)?;

    let path = "file:/home/user/test.txt";
    println!("file open: {}", path);
    let socket_fd = syscall::open(path, syscall::O_RDWR).map_err(from_syscall_error)?;

    println!("sendfd");
    let res = syscall::sendfd(sender_fd, socket_fd, 0, 0).map_err(from_syscall_error)?;

    println!("res: {}", res);

    Ok(())
}
