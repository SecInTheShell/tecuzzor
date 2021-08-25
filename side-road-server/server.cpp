#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <errno.h>

#include <arpa/inet.h>
#include <sys/socket.h>
#include <netinet/in.h>

#define MAXLINE 24
#define MAXCONN 256

int main()
{
	int listenfd, connfd;
	struct sockaddr_in sockaddr;
	// char buff[MAXLINE];
	unsigned long payload[MAXLINE / 8];
	int n;

	unsigned long A_a, A_b;
	int pid;

	memset(&sockaddr, 0, sizeof(sockaddr));

	sockaddr.sin_family = AF_INET;
	sockaddr.sin_addr.s_addr = inet_addr("127.0.0.1");
	sockaddr.sin_port = htons(1234);

	listenfd = socket(AF_INET, SOCK_STREAM, 0);

	bind(listenfd, (struct sockaddr *)&sockaddr, sizeof(sockaddr));

	listen(listenfd, MAXCONN);

	printf("Please wait for the client information\n");

	for (;;)
	{
		if ((connfd = accept(listenfd, (struct sockaddr *)NULL, NULL)) == -1)
		{
			printf("accpet socket error: %s errno :%d\n", strerror(errno), errno);
			continue;
		}

		// n = recv(connfd, buff, MAXLINE, 0);
		// buff[n] = '\0';
		// printf("recv msg from client:%s\n", buff);
		printf("Receiving info...\n");
		recv(connfd, payload, MAXLINE, 0);
		printf("Address a: 0x%lx, Address b: 0x%lx\n", payload[1], payload[2]);
		printf("pid: %ld\n", payload[0]);
		pid = (int)payload[0];
		A_a = payload[1];
		A_b = payload[2];
		close(connfd);

		printf("Sending the info to DebugFS...\n");
		FILE *fp = NULL;
		//WL: need to deal with the exceptions

		fp = fopen("/sys/kernel/debug/middleware/addr", "w+");
		if (fp == NULL)
		{
			printf("Something wrong with the Debugfs!\n");
			break;
		}
		fprintf(fp, "0x%lx 0x%lx", A_a, A_b);
		fclose(fp);

		//WL: we pass pid to trigger the km, after passing addresses.
		fp = fopen("/sys/kernel/debug/middleware/pids", "w+");
		if (fp == NULL)
		{
			printf("Something wrong with the DebugFS!\n");
			break;
		}
		fprintf(fp, "%d\n", pid);
		fclose(fp);

		printf("Sending the info successfully\n");
	}

	close(listenfd);

	return 0;
}
