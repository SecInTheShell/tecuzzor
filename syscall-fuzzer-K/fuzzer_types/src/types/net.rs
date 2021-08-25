//! Types Related to Network

use super::*;

#[cfg(feature = "type_only")]
pub type Domain = c_int;

#[cfg(not(feature = "type_only"))]
#[repr(C)]
#[derive(Debug, Serialize)]
pub struct Domain(c_int);

use libc::{
    AF_DECnet, AF_ALG, AF_APPLETALK, AF_ASH, AF_ATMPVC, AF_ATMSVC, AF_AX25, AF_BLUETOOTH,
    AF_BRIDGE, AF_CAIF, AF_CAN, AF_ECONET, AF_IEEE802154, AF_INET, AF_INET6, AF_IPX, AF_IRDA,
    AF_ISDN, AF_IUCV, AF_KEY, AF_LLC, AF_LOCAL, AF_NETBEUI, AF_NETLINK, AF_NETROM, AF_PACKET,
    AF_PHONET, AF_PPPOX, AF_RDS, AF_ROSE, AF_ROUTE, AF_RXRPC, AF_SECURITY, AF_SNA, AF_TIPC,
    AF_UNIX, AF_UNSPEC, AF_WANPIPE, AF_X25,
};

#[cfg(feature = "type_only")]
pub type SockType = c_int;

#[cfg(not(feature = "type_only"))]
#[repr(C)]
#[derive(Debug, Serialize)]
pub struct SockType(c_int);

use libc::{
    SOCK_CLOEXEC, SOCK_DCCP, SOCK_DGRAM, SOCK_NONBLOCK, SOCK_PACKET, SOCK_RAW, SOCK_RDM,
    SOCK_SEQPACKET, SOCK_STREAM,
};


#[cfg(feature = "type_only")]
pub type Protocol = c_int;

#[cfg(not(feature = "type_only"))]
#[repr(C)]
#[derive(Debug, Serialize)]
pub struct Protocol(c_int);

#[cfg(feature = "type_only")]
pub type SockFd = c_int;

#[cfg(not(feature = "type_only"))]
#[repr(C)]
#[derive(Debug, Serialize)]
pub struct SockFd(pub c_int);

// #[repr(C)]
// pub struct sockaddr {
//     pub sa_family: c_ushort,
//     pub sa_data: [c_char; 14],
// }

#[cfg(feature = "type_only")]
pub type MsgFlag = c_int;

#[cfg(not(feature = "type_only"))]
#[repr(C)]
#[derive(Debug, Serialize)]
pub struct MsgFlag(c_int);

use libc::{
    MSG_CMSG_CLOEXEC, MSG_CONFIRM, MSG_CTRUNC, MSG_DONTROUTE, MSG_DONTWAIT, MSG_EOR, MSG_ERRQUEUE,
    MSG_FASTOPEN, MSG_FIN, MSG_MORE, MSG_NOSIGNAL, MSG_OOB, MSG_PEEK, MSG_RST, MSG_SYN, MSG_TRUNC,
    MSG_WAITALL, MSG_WAITFORONE,
};

#[cfg(feature = "type_only")]
pub type Level = c_int;

#[cfg(not(feature = "type_only"))]
#[repr(C)]
#[derive(Debug, Serialize)]
pub struct Level(c_int);

use libc::{
    SOL_AAL, SOL_ALG, SOL_ATM, SOL_BLUETOOTH, SOL_DCCP, SOL_DECNET, SOL_ICMPV6, SOL_IP, SOL_IPV6,
    SOL_IRDA, SOL_LLC, SOL_NETBEUI, SOL_NETLINK, SOL_PACKET, SOL_RAW, SOL_SOCKET, SOL_TCP,
    SOL_TIPC, SOL_UDP, SOL_X25,
};

#[cfg(feature = "type_only")]
pub type OptName = c_int;

#[cfg(not(feature = "type_only"))]
#[repr(C)]
#[derive(Debug, Serialize)]
pub struct OptName(c_int);

use libc::{
    SCM_TIMESTAMPING_OPT_STATS, SCM_TIMESTAMPING_PKTINFO, SO_ACCEPTCONN, SO_ATTACH_BPF,
    SO_ATTACH_FILTER, SO_ATTACH_REUSEPORT_CBPF, SO_ATTACH_REUSEPORT_EBPF, SO_BINDTODEVICE,
    SO_BINDTOIFINDEX, SO_BPF_EXTENSIONS, SO_BROADCAST, SO_BSDCOMPAT, SO_BUSY_POLL, SO_CNX_ADVICE,
    SO_COOKIE, SO_DEBUG, SO_DETACH_FILTER, SO_DOMAIN, SO_DONTROUTE, SO_ERROR, SO_INCOMING_CPU,
    SO_INCOMING_NAPI_ID, SO_KEEPALIVE, SO_LINGER, SO_LOCK_FILTER, SO_MARK, SO_MAX_PACING_RATE,
    SO_MEMINFO, SO_NOFCS, SO_NO_CHECK, SO_OOBINLINE, SO_PASSCRED, SO_PASSSEC, SO_PEEK_OFF,
    SO_PEERCRED, SO_PEERGROUPS, SO_PEERNAME, SO_PEERSEC, SO_PRIORITY, SO_PROTOCOL, SO_RCVBUF,
    SO_RCVBUFFORCE, SO_RCVLOWAT, SO_RCVTIMEO, SO_REUSEADDR, SO_REUSEPORT, SO_RXQ_OVFL,
    SO_SECURITY_AUTHENTICATION, SO_SECURITY_ENCRYPTION_NETWORK, SO_SECURITY_ENCRYPTION_TRANSPORT,
    SO_SELECT_ERR_QUEUE, SO_SNDBUF, SO_SNDBUFFORCE, SO_SNDLOWAT, SO_SNDTIMEO, SO_TIMESTAMP,
    SO_TIMESTAMPING, SO_TIMESTAMPNS, SO_TXTIME, SO_TYPE, SO_WIFI_STATUS, SO_ZEROCOPY,
};

