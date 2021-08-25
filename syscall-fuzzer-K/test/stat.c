#include <stdio.h>
#include <sys/stat.h>
#include <unistd.h>
int main()
{
    int pid = getpid();
    printf("%d\n", pid);
    
    FILE *fp = NULL;
    fp = fopen("/sys/kernel/debug/middleware/pids", "w+");
    fprintf(fp, "%d", pid);
    fclose(fp);

    printf("staring...\n");
    
    struct stat buf;
    stat("/etc/passwd", &buf);
    printf("/etc/passwd file size = %ld \n", buf.st_size);
    
    return 0;
}
