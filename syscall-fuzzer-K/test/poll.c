#include <sys/socket.h>
#include <sys/epoll.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <fcntl.h>
#include <unistd.h>
#include <string.h>
#include <errno.h>
#include <sys/types.h>
#include <stdio.h>

#define    MAXLINE        4096
#define    LISTENQ        20
#define    SERV_PORT    9877

int main(int argc, char* argv[])
{
    int pid = getpid();
    printf("%d\n", pid);
    
    FILE *fp = NULL;
    fp = fopen("/sys/kernel/debug/middleware/pids", "w+");
    fprintf(fp, "%d", pid);
    fclose(fp);

    printf("staring...\n");
    
    int i, maxi, listenfd, connfd, sockfd, epfd,nfds;
    ssize_t n;
    char BUF[MAXLINE];
    socklen_t clilen;

    //ev用于注册事件,数组用于回传要处理的事件

    struct epoll_event ev,events[20];
    //生成用于处理accept的epoll专用的文件描述符

    epfd=epoll_create(256);
    struct sockaddr_in cliaddr, servaddr;
    listenfd = socket(AF_INET, SOCK_STREAM, 0);
 
    //setnonblocking(listenfd);
    //设置与要处理的事件相关的文件描述符

    ev.data.fd=listenfd;
    ev.events=EPOLLIN|EPOLLET;
    //注册epoll事件

    epoll_ctl(epfd,EPOLL_CTL_ADD,listenfd,&ev);

    bzero(&servaddr, sizeof(servaddr));
    servaddr.sin_family = AF_INET;
    servaddr.sin_addr.s_addr = htonl (INADDR_ANY);
    servaddr.sin_port = htons (SERV_PORT);
    bind(listenfd, (struct sockaddr *)&servaddr, sizeof(servaddr));
    listen(listenfd, LISTENQ);
    maxi = 0; //useless
    for ( ; ; ) {
        nfds=epoll_wait(epfd,events,20,0);

        for(i=0;i<nfds;++i)
        {
            if(events[i].data.fd==listenfd)//如果新监测到一个SOCKET用户连接到了绑定的SOCKET端口，建立新的连接。
            {
                connfd = accept(listenfd,(struct sockaddr *)&cliaddr, &clilen);
                if(connfd<0){
                    perror("connfd<0");
                    exit(1);
                }
                //setnonblocking(connfd);

                char *str = inet_ntoa(cliaddr.sin_addr);
                printf("accapt a connection from %s\n", str);

                ev.data.fd=connfd;
                ev.events=EPOLLIN|EPOLLET;
                epoll_ctl(epfd,EPOLL_CTL_ADD,connfd,&ev);
            }
            else if (events[i].events&EPOLLIN)
            //如果是已经连接的用户，并且收到数据，那么进行读入。

            {
                printf("EPOLLIN\n");
                if ( (sockfd = events[i].data.fd) < 0)
                    continue;
                if ( (n = read(sockfd, BUF, MAXLINE)) < 0) {
                    if (errno == ECONNRESET) {
                        close(sockfd);
                        events[i].data.fd = -1;
                    } else
                        printf("readline error\n");
                } else if (n == 0) {
                    close(sockfd);
                    events[i].data.fd = -1;
                }
                BUF[n] = '\0';
                printf("AFTER EPOLLIN\n");

                ev.data.fd=sockfd;
                ev.events=EPOLLOUT|EPOLLET;
                //读完后准备写
                epoll_ctl(epfd,EPOLL_CTL_MOD,sockfd,&ev);

            }
            else if(events[i].events&EPOLLOUT) // 如果有数据发送

            {
                sockfd = events[i].data.fd;
                write(sockfd, BUF, n);

                ev.data.fd=sockfd;
                ev.events=EPOLLIN|EPOLLET;
                //写完后，这个sockfd准备读
                epoll_ctl(epfd,EPOLL_CTL_MOD,sockfd,&ev);
            }
        }
    }
    return 0;
}
