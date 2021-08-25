#include <fcntl.h>
#include <signal.h>
#include <stdio.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>

static int alloc_size;
static char* memory;

void segv_handler(int signal_number)
{
	printf("find memory accessed!\n");
	mprotect(memory, alloc_size, PROT_READ | PROT_WRITE);
	
	printf("set memory read write!\n");
}

int main()
{
	int pid = getpid();
    printf("%d\n", pid);
    
    FILE *fp = NULL;
    fp = fopen("/sys/kernel/debug/middleware/pids", "w+");
    fprintf(fp, "%d", pid);
    fclose(fp);

    printf("staring...\n");
	
	int fd;
	struct sigaction sa;

	
	memset(&sa, 0, sizeof(sa));
	sa.sa_handler = &segv_handler;
	sigaction(SIGSEGV, &sa, NULL);

	 
	alloc_size = getpagesize();
	fd = open("/dev/zero", O_RDONLY);
	memory = mmap(NULL, alloc_size, PROT_WRITE, MAP_PRIVATE, fd, 0);
	close(fd);
	
	memory[0] = 0;
	printf("memory[0] = 0\n");
	
	mprotect(memory, alloc_size, PROT_NONE);
	printf("memory[0] = 1 SIGSEGV\n");
	
	memory[0] = 1;
	printf("memory[0] = 2 ok\n");
	
	memory[0] = 2;
	
	printf("all done\n");
	munmap(memory, alloc_size);
	return 0;
}
