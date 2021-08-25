#include <fcntl.h>
#include <unistd.h>
#include <stdio.h>
 
#define BUFFSIZE 20
int main(void)
{
    int pid = getpid();
    printf("%d\n", pid);
    
    FILE *fp = NULL;
    fp = fopen("/sys/kernel/debug/middleware/pids", "w+");
    fprintf(fp, "%d", pid);
    fclose(fp);

    printf("staring...\n");
    
    char        pathname[] = "/tmp/myfile";   /*待操作文件路径*/
    int         f_id;                         /*文件描述符*/
 
    off_t       f_offset;                     /*文件指针偏移量*/
 
    ssize_t     nwrite;                       /*实际写入的字节数*/
    char        buf[BUFFSIZE] = "0123456789abcd"; /*待写入数据*/
    size_t      nbytes;                       /*准备写入的字节数*/
 
    /*打开文件，获取文件标识符*/
    f_id = open(pathname, O_RDWR | O_CREAT);
    if (f_id == -1) {
        printf("open error for %s\n", pathname);
        return 1;
    }
 
    /*把文件指针移动到文件开始处*/
    f_offset = lseek(f_id, 0, SEEK_SET);
    if (f_offset == -1) {
        printf("lseek error for %s\n", pathname);
        return 2;
    }
 
    /*=======调用pwrite从第一个字节后面写入四个字节数据[abcd]=======*/
    nbytes = 4;
    nwrite = pwrite(f_id, (buf + 10), nbytes, 1);
    if (nwrite == -1) {
        printf("pwrite error for %s\n", pathname);
        return 4;
    }
 
    /*关闭文件*/
    close(f_id);
    printf("succeed.\n");
    return 0;
}