#include <stdio.h>
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
    
    if(access("/etc/passwd", R_OK) == 0)
    printf("/etc/passwd can be read\n");
}
