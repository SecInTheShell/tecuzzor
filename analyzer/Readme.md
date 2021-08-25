
## An example - poll

### u-part print-out

ratel@ratel-ThinkPad-11e-3rd-Gen:~/ratel$ ./ratel -- fuzz_test/syscall_fuzzer 
begin
fuzzer_types::calls::fs::Poll: Poll { fds: ArgBuffer([PollFd { fd: Fd(3), events: PollEvent(512), revents: PollEvent(0) }, PollFd { fd: Fd(4), events: PollEvent(64), revents: PollEvent(0) }]), nfds: BufferLength(2), timeout: TimeMilliSec(1957) }
---- result from fuzzer_types::calls::fs::Poll syscall: Ok(1)
---- after poll: Poll { fds: ArgBuffer([PollFd { fd: Fd(3), events: PollEvent(512), revents: PollEvent(0) }, PollFd { fd: Fd(4), events: PollEvent(64), revents: PollEvent(0) }]), nfds: BufferLength(2), timeout: TimeMilliSec(1957) }
end
begin
fuzzer_types::calls::fs::Poll: Poll { fds: ArgBuffer([PollFd { fd: Fd(3), events: PollEvent(16), revents: PollEvent(0) }, PollFd { fd: Fd(4), events: PollEvent(64), revents: PollEvent(0) }, PollFd { fd: Fd(6), events: PollEvent(256), revents: PollEvent(0) }]), nfds: BufferLength(3), timeout: TimeMilliSec(1642) }
---- result from fuzzer_types::calls::fs::Poll syscall: Ok(1)
---- after poll: Poll { fds: ArgBuffer([PollFd { fd: Fd(3), events: PollEvent(16), revents: PollEvent(0) }, PollFd { fd: Fd(4), events: PollEvent(64), revents: PollEvent(0) }, PollFd { fd: Fd(6), events: PollEvent(256), revents: PollEvent(0) }]), nfds: BufferLength(3), timeout: TimeMilliSec(1642) }
end
begin
fuzzer_types::calls::fs::Poll: Poll { fds: ArgBuffer([]), nfds: BufferLength(0), timeout: TimeMilliSec(856) }
---- result from fuzzer_types::calls::fs::Poll syscall: Err(1)
---- after poll: Poll { fds: ArgBuffer([]), nfds: BufferLength(0), timeout: TimeMilliSec(856) }
end

### Example k-part output

ratel@ratel-ThinkPad-11e-3rd-Gen:~/SGX-middlewares-survey/ftrace-hooking$ sudo dmesg -c

[ 5522.978152] [hooking] func: poll, type: input, *ufds: 00000000754badfd, pollfd->fd0: 0, polled->events0: 0, pollfd->fd1: 289, polled->events1: 2048, nfds: 3, timeout: 0
[ 5523.161304] [hooking] func: poll, type: input, *ufds: 00000000754badfd, pollfd->fd0: 3, polled->events0: 512, pollfd->fd1: 539572021, polled->events1: 2685, nfds: 2, timeout: 1957
[ 5527.193768] [hooking] func: poll, type: input, *ufds: 00000000754badfd, pollfd->fd0: 3, polled->events0: 16, pollfd->fd1: 1768383842, polled->events1: 2670, nfds: 3, timeout: 1642
