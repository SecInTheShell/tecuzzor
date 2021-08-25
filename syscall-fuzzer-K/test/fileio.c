#include <stdio.h>
#include <unistd.h>

#include <fcntl.h>
#include <sys/types.h>
#include <sys/stat.h>

int main()
{
	int pid = getpid();
    printf("%d\n", pid);
    
    FILE *fp_tmp = NULL;
    fp_tmp = fopen("/sys/kernel/debug/middleware/pids", "w+");
    fprintf(fp_tmp, "%d", pid);
    fclose(fp_tmp);

	// sleep(5);

    printf("staring...\n");
	
	FILE *fp = NULL;
	char buff[50];
	
	// open
	// fp = fopen("test.txt", "rw+");
	int fd = open("test.txt", O_RDWR);
	// printf("[stage1]: open file\n");
     	
	// read
	// fscanf(fp, "%s", buff);
	// printf("[stage1]: read file: %s\n", buff);
	read(fd, buff, 50);
	buff[49] = '\0';
	printf("buff: %s\n", buff);

	// write
	// fprintf(fp, "This is testing for fprintf...\n");
	// printf("[stage1]: write file\n");
	write(fd, "jie ge hen da!", 50);
	
	// close
	// fclose(fp);
	close(fd);

	return 0;
}
