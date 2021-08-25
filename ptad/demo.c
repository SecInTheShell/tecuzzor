#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>
#include <unistd.h>
#include <sys/socket.h>
#include <arpa/inet.h>

#define LOOP INT_MAX
#define PACKET_LEN 24

/* see asm.S */
extern int a;
extern int b;

void inc_secret(int s)
{
	if (s)
		a += 1;
	else
		b += 1;
}

void send_info(unsigned long pid, unsigned long address_a, unsigned long address_b)
{
	int sock = socket(AF_INET, SOCK_STREAM, 0);

	struct sockaddr_in serv_addr;
	memset(&serv_addr, 0, sizeof(serv_addr));			//
	serv_addr.sin_family = AF_INET;						//
	serv_addr.sin_addr.s_addr = inet_addr("127.0.0.1"); //
	serv_addr.sin_port = htons(1234);					//
	connect(sock, (struct sockaddr *)&serv_addr, sizeof(serv_addr));

	long payload[PACKET_LEN / 2];
	//Format: pid (8 bytes), a (8 bytes), b (8 bytes).
	payload[0] = pid;
	payload[1] = address_a;
	payload[2] = address_b;

	write(sock, payload, sizeof(payload));

	close(sock);
}

//WL: e.g., Graphene gives tweaked pid
//When pid is tweaked or not supported, we need to pass the pid manually
#define PID_SUPPORT 1

#if PID_SUPPORT
#define getrealpid() getpid()
#else
#define getreadpid() 0
#endif

int main(void)
{
	//WL: access a, b. pages loaded
	printf("Address of a: %p\n", &a);
	printf("Address of b: %p\n", &b);
	
	
	int pid = getrealpid();

	printf("Sending to the side-road server...\n");
	send_info((unsigned long)pid, (unsigned long)&a, (unsigned long)&b);
	//WL: for kernel module to set the PTE bits

	//WL: have to sleep enough time in case of setting pid manually
	printf("Message sent!\nPlease set the pid ASAP if getpid() is tweaked!\n");
	sleep(5);

	//WL: bits are set.
	int i, j;
	printf("Start to loop\n");
	for (j = 0; j < 100; j++)
	{
		printf("Round %d\n", j);
		for (i = 0; i < LOOP; i++)
		{
			inc_secret(5);
			inc_secret(-6);
		}
		a = 0;
		b = 0;
		// printf("Sleeping...\n");
		// sleep(1);
	}

	return 0;
}
