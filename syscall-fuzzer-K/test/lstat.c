#include <stdio.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>
#include <fcntl.h>


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
    struct stat buf[3];
   
    // stat函数
    if( 0 != stat("/etc/passwd", &buf[0]))
    {
        printf("stat error!\n");
        return -1;
    }
    printf("stat: The file size is %lu\n", buf[0].st_size);


    // fstat函数
    fd = open("/etc/passwd", O_RDWR);
    if(fd < 0)
    {
        printf("open file error!\n");
        return -1;
    }
    if(0 != fstat(fd, &buf[1]))
    {
        printf("fstat error!\n");
        return -1;
    }
    printf("fstat: The file size is %lu\n", buf[1].st_size);
    close(fd);
   

    // lstat函数
    if(0 != lstat("/etc/passwd", &buf[2]))
    {
        printf("lstat error!\n");
        return -1;
    }
    printf("lstat: The file size is %lu\n", buf[2].st_size);


    // 比较stat和lstat
    if(S_ISLNK(buf[0].st_mode))
        printf("stat: It is a link file!\n");
    else
        printf("stat: It is not a link file!\n");

    if(buf[2].st_mode & S_IFLNK)
        printf("lstat: It is a link file!\n");
    else
        printf("lstat: It is not a link file!\n");

    return 0;  
}
