#include <fcntl.h>
#include <unistd.h>
#include <stdio.h>
 
#define BUFFSIZE 256
int main(void)
{
    int pid = getpid();
    printf("%d\n", pid);
    
    FILE *fp = NULL;
    fp = fopen("/sys/kernel/debug/middleware/pids", "w+");
    fprintf(fp, "%d", pid);
    fclose(fp);

    printf("staring...\n");
    
    char        pathname[] = "./test.txt";  /*文件路径*/
    int         f_id;                        /*文件标识符*/
 
    off_t       f_offset;                    /*文件指针偏移量*/
 
    ssize_t     nread;                       /*实际读取的字节数*/
    char        buf[BUFFSIZE];               /*数据缓冲器*/
    size_t      nbytes;                      /*要读取的字节数*/
 
    /*打开文件，获得文件标识符*/
    f_id = open(pathname, O_RDONLY);
    if (f_id == -1) {
        printf("open error for %s\n", pathname);
        return 1;
    }
 
    /*设置文件指针*/
    f_offset = lseek(f_id, 0, SEEK_SET);
    if (f_offset == -1) {
        printf("lseek error for %s\n", pathname);
        return 2;
    }
 
    /*===调用pread读取10个字节====*/
    nbytes = 10;    
    nread = pread(f_id, buf, nbytes, 0);
    if (nread == -1) {
        printf("read error for %s\n", pathname);
        return 3;
    }
    nread = nread == 0 ? nbytes : nread;
    buf[nread] = '\0';
    printf("byte[0-9](pread):%s\n", buf);
 
    /*关闭文件*/
    close(f_id);
 
    return 0;
}

