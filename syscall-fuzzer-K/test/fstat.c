#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <unistd.h>

int main(void)
{
   int pid = getpid();
   printf("%d\n", pid);
    
   FILE *fp = NULL;
   fp = fopen("/sys/kernel/debug/middleware/pids", "w+");
   fprintf(fp, "%d", pid);
   fclose(fp);

   printf("staring...\n");

   struct stat buf;
   int fd;
   fd = open("/etc/passwd", O_RDONLY);
   fstat(fd, &buf);
   printf("/etc/passwd file size %ld\n ", buf.st_size);
   
   return 0 ;
}
