//! Syscalls Related to Networks

use super::*;
// use crate::types::net::Protocol;

/// `int socket(int domain, int type, int protocol);`  
/// socket - create an endpoint for communication  
/// [Linux Manual: socket](https://man7.org/linux/man-pages/man2/socket.2.html)
#[derive(Debug, Serialize)]
pub struct Socket {
    pub domain: Domain,
    pub sock_type: SockType,
    pub protocol: Protocol,
}

/// `int connect(int sockfd, const struct sockaddr *addr, socklen_t addrlen);`  
/// connect - initiate a connection on a socket  
/// [Linux Manual: socket](https://man7.org/linux/man-pages/man2/Connect.2.html)
/// **Warning**: This system call may not always guarantee correct `addr`.  
/// TODO: clean
#[derive(Debug, Serialize)]
pub struct Connect {
    pub sockfd: SockFd,
    pub addr: ArgBuffer::<u8>,
    pub addrlen: BufferLength,
}

/// `ssize_t send(int sockfd, const void *buf, size_t len, int flags);`  
/// send, sendto, sendmsg - send a message on a socket  
/// [Linux Manual: send](https://man7.org/linux/man-pages/man2/send.2.html)  
/// There is no `send` syscall. It will be convert to
/// `sendto(sockfd, buf, len, flags, NULL, 0);`  
#[derive(Debug, Serialize)]
pub struct Send {
    pub sockfd: SockFd,
    pub buf: ArgBuffer::<u8>,
    pub len: BufferLength,
    pub flags: MsgFlag,
}

/// `ssize_t sendto(int sockfd, const void *buf, size_t len, int flags, const struct sockaddr *dest_addr, socklen_t addrlen);`
#[derive(Debug, Serialize)]
pub struct SendTo {
    pub sockfd: SockFd,
    pub buf: ArgBuffer<u8>,
    pub len: BufferLength,
    pub flags: MsgFlag,
    pub dest_addr: ArgBuffer<u8>,
    pub addrlen: BufferLength,
}


/// recv, recvfrom, recvmsg - receive a message from a socket  
/// `ssize_t recv(int sockfd, void *buf, size_t len, int flags);`  
/// [Linux Manual: recv](https://man7.org/linux/man-pages/man2/recv.2.html)  
/// There is no `recv` syscall. It will be convert to
/// `recvfrom(sockfd, buf, len, flags, NULL, 0);`
#[derive(Debug, Serialize)]
pub struct Recv {
    pub sockfd: SockFd,
    pub buf: RetBuffer::<u8>,
    pub len: BufferLength,
    pub flags: MsgFlag,
}


/// `ssize_t recvfrom(int sockfd, void *restrict buf, size_t len, int flags, struct sockaddr *restrict src_addr, socklen_t *restrict addrlen);`
#[derive(Debug, Serialize)]
pub struct RecvFrom {
    pub sockfd: SockFd,
    pub buf: RetBuffer<u8>,
    pub len: BufferLength,
    pub flags: MsgFlag,
    pub src_addr: ArgBuffer<u8>,
    pub addrlen: BufferLength,
}

/// getsockopt, setsockopt - get and set options on sockets  
/// `int getsockopt(int sockfd, int level, int optname, void *restrict optval, socklen_t *restrict optlen);`  
/// [Linux Manual: socket](https://man7.org/linux/man-pages/man2/Send.2.html)  
#[derive(Debug, Serialize)]
pub struct Getsockopt {
    socket: SockFd,
    level: Level,
    optname: OptName,
    optval: RetBuffer::<u8>,
    optlen: Rc<libc::socklen_t>,
}


/// `int setsockopt(int sockfd, int level, int optname, const void *optval, socklen_t optlen);`
#[derive(Debug, Serialize)]
pub struct Setsockopt {
    socket: SockFd,
    level: Level,
    optname: OptName,
    optval: ArgBuffer::<u8>,
    optlen: BufferLength,
}