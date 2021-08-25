#include <stdio.h>
#include <sys/uio.h>
int main(int argc,char **argv)
{
    int pid = getpid();
    printf("%d\n", pid);
    
    FILE *fp = NULL;
    fp = fopen("/sys/kernel/debug/middleware/pids", "w+");
    fprintf(fp, "%d", pid);
    fclose(fp);

    printf("staring...\n");
    
    char part1[] = "This is iov";
    char part2[] = " and ";
    char part3[] = " writev test";

    struct iovec iov[3];
    iov[0].iov_base = part1;
    iov[0].iov_len = strlen(part1);
    iov[1].iov_base = part2;
    iov[1].iov_len = strlen(part2);
    iov[2].iov_base = part3;
    iov[2].iov_len = strlen(part3);
    writev(1,iov,3);
    return 0;
}
