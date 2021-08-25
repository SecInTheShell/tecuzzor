//! Fuzz syscalls related networks  

use super::*;
use dec_macro::{call, testcall, type_of};
use fuzzer_types::*;
use rand::rngs::StdRng;

pub fn net_test(gen: &mut StdRng) {
    for _ in 0..REPEAT {
        // socket
        let (socket, res) = testcall!(Socket, gen);
        println!("---- after {}: {}", type_of(&socket), serde_json::to_string(&socket).unwrap());
        
        #[cfg(not(feature = "type_only"))]
        socket.clean(res);

        // connect
        let (connect, _res) = testcall!(Connect, gen);
        println!("---- after {}: {}", type_of(&connect), serde_json::to_string(&connect).unwrap());

        // send
        let (send, _res) = testcall!(Send, gen);
        println!("---- after {}: {}", type_of(&send), serde_json::to_string(&send).unwrap());

        // sendto
        let (sendto, _res) = testcall!(SendTo, gen);
        println!("---- after {}: {}", type_of(&sendto), serde_json::to_string(&sendto).unwrap());

        // // recv
        // // This syscall may block
        // let (recv, _res) = testcall!(Recv, gen);
        // println!("---- after lstat: {:?}", recv);

        // recvfrom
        let (recvfrom, _res) = testcall!(RecvFrom, gen);
        println!("---- after {}: {}", type_of(&recvfrom), serde_json::to_string(&recvfrom).unwrap());

        // getsockopt
        // This syscall can succeed with a low probability 
        let (getsockopt, _res) = testcall!(Getsockopt, gen);
        println!("---- after {}: {}", type_of(&getsockopt), serde_json::to_string(&getsockopt).unwrap());

        // setsockopt
        // This syscall can succeed with a low probability 
        let (setsockopt, _res) = testcall!(Setsockopt, gen);
        println!("---- after {}: {}", type_of(&setsockopt), serde_json::to_string(&setsockopt).unwrap());
    }
}
