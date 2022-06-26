use socket::socket::*;
use std::{ffi::c_void, mem};

pub fn daytimetcpcli(addr: &str) {
    let mut recvline = [0u8; (MAXLINE as usize) + 1];

    unsafe {
        let sockfd = socket(AF_INET, SOCK_STREAM, 0);

        if sockfd < 0 {
            panic!("{}", "socket error");
        }

        // todo inet_pton()
        let addr_vec = addr
            .split(".")
            .map(|s| s.parse::<u8>().unwrap())
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        let s_addr = u32::from_be_bytes(addr_vec).to_be();
        let servaddr = sockaddr_in {
            sin_family: AF_INET as u16,
            sin_port: 13u16.to_be(),
            sin_addr: in_addr { s_addr },
            sin_zero: mem::zeroed(),
        };

        let conn_st = connect(
            sockfd,
            &servaddr as *const sockaddr_in as *const sockaddr,
            mem::size_of_val(&servaddr) as u32,
        );

        if conn_st < 0 {
            panic!("{}", "connect error");
        }

        let n = read(
            sockfd,
            &mut recvline as *mut _ as *mut c_void,
            MAXLINE as usize,
        );
        if n < 0 {
            panic!("{}", "read error");
        }
        println!("{:?}", String::from_utf8_lossy(&recvline[0..n as usize]));
    }
}

#[test]
pub fn client() {
    let addr = "127.0.0.1";
    daytimetcpcli(addr);
}
