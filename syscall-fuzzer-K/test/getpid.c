# include <stdio.h>
# include <unistd.h>

int main()
{
    int pid = getpid();
    printf("%d\n", pid);
    
    FILE *fp = NULL;
    fp = fopen("/sys/kernel/debug/middleware/pids", "w+");
    fprintf(fp, "%d", pid);
    fclose(fp);

    printf("staring...\n");
    
    printf("pid=%d\n", getpid());
    return 0;
}
