pub mod c_type {
    use std::os::raw::c_char;

    pub const AF_INET: i32 = 2;
    pub const AF_INET6: i32 = 10;
    pub const SOCK_STREAM: i32 = 1;
    pub const IPPRPTO_TCP: i32 = 6;

    // IPV4套接字地址
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct in_addr {
        pub s_addr: u32, // 32bit ipv4地址
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct sockaddr_in {
        pub sin_len: u8,
        pub sin_family: u16,
        pub sin_port: u16,
        pub sin_addr: in_addr,
        pub sin_zero: [u8; 8],
    }

    // 通用套接字地址
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct sockaddr {
        pub sa_len: u8,
        pub sa_family: u16,
        pub sa_data: [c_char; 14],
    }

    // IPV6套接字地址
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct in6_addr {
        pub s6_addr: [u8; 16], // 126bit IPV6地址
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct sockaddr_in6 {
        pub sin6_len: u8,
        pub sin6_family: u16,
        pub sin6_port: u16,
        pub sin6_flowinfo: u32,  // 流信息
        pub sin6_addr: in6_addr, // IPV6地址
        pub sin6_scope_id: u32,  // set of interface for a scope
    }

    // 新的通用套接字地址结构
    pub struct sockaddr_storage {
        pub ss_len: u8,
        pub ss_family: u16,
        _unused: [u8; 126],
    }

    extern "C" {
        pub fn socket(family: i32, tp: i32, protocol: i32) -> i32;
        pub fn connect(sockfd: i32, servaddr: *const sockaddr, addrlen: u32) -> i32;
        // pub fn bind(sockfd: i32, )
    }
}
