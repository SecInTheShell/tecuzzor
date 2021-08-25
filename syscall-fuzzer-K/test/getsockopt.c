#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <errno.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <assert.h>

#include <netinet/in.h>
#include <netinet/tcp.h>

static void bail(const char *on_what)
{
    if (errno != 0)
    {
        fputs(strerror(errno), stderr);
        fputs(": ", stderr);
    }
    fputs(on_what, stderr);
    fputc('\n', stderr);
    exit(1);
}

int main(int argc, char **argv)
{
    int pid = getpid();
    printf("%d\n", pid);

    FILE *fp = NULL;
    fp = fopen("/sys/kernel/debug/middleware/pids", "w+");
    fprintf(fp, "%d", pid);
    fclose(fp);

    printf("staring...\n");

    int z;
    int s = -1;
    int sndbuf = 0;
    int rcvbuf = 0;
    socklen_t optlen;

    s = socket(PF_INET, SOCK_STREAM, 0);
    if (s == -1)
        bail("socket(2)");

    // 设置心跳包发送时间间隔(即两个心跳包之间的发送时间间隔)
    int interval = 60;
    if (setsockopt(s, IPPROTO_TCP, TCP_KEEPINTVL, &interval, sizeof(interval)) < 0)
    {
        perror("\033[1;35m[WARNING]\033[0m setsockopt_keepintvl");
    }
    
    optlen = sizeof sndbuf;
    z = getsockopt(s, SOL_SOCKET, SO_SNDBUF, &sndbuf, &optlen);

    if (z)
        bail("getsockopt(s,SOL_SOCKET,"
             "SO_SNDBUF)");

    assert(optlen == sizeof sndbuf);

    optlen = sizeof rcvbuf;
    z = getsockopt(s, SOL_SOCKET, SO_RCVBUF, &rcvbuf, &optlen);
    if (z)
        bail("getsockopt(s,SOL_SOCKET,"
             "SO_RCVBUF)");

    assert(optlen == sizeof rcvbuf);

    printf("Socket s: %d/n", s);
    printf("Send buf: %d bytes/n", sndbuf);
    printf("Recv buf: %d bytes/n", rcvbuf);

    close(s);
    return 0;
}
