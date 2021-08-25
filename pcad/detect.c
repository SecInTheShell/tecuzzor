#include <stdio.h>
#include <unistd.h>
#include <fcntl.h>
#include <stdlib.h>
#include <sys/mman.h>
#include <errno.h>
#include <dirent.h>
#include <pthread.h>
#include <string.h>
#include <time.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>

#define s_t unsigned long long

//char targetfileName[] = "/home/lys/Documents/pagecachefiles/targetfile%d";
char targetfileName[] = "/home/liuweijie/Documents/pagecachefiles/targetfile%d";

float detect_targetfile(char targetfileName[])
{
    int fd_target = open(targetfileName, O_RDONLY);
    if (fd_target == -1)
    {
        return -1;
    }
    s_t size_target = (s_t)lseek(fd_target, 0, SEEK_END);
    int pc_target = size_target / (4096);
    unsigned char v[pc_target];
    void* addr_target = mmap(NULL, size_target, PROT_READ, MAP_SHARED, fd_target, 0);
    mincore(addr_target, size_target, v);
    int count = 0;
    float percent = 0;
    for (int i=0; i<pc_target; i++){
        if (v[i] == 1)
        {
            count++;
        }
    }
    if (count == 0)
        return 0;
    else if (count == pc_target)
    {
        return 1;
    }
    else
    {
        percent = (float)count/pc_target;
        return percent;
    }
}

int main()
{
    char targetfileNameArray[3][100];
    sprintf(targetfileNameArray[0], targetfileName, 1);
    sprintf(targetfileNameArray[1], targetfileName, 2);
    sprintf(targetfileNameArray[2], targetfileName, 3);
    
    float d[3] = {0, 0, 0};
    int b = 0;
    for (int i = 0; i < 3; i++)
    {
        d[i] = detect_targetfile(targetfileNameArray[i]);
        if (d[i] > 0.5)
        {
            b = 1;
            printf("%d\n", i+1);
        }
    }
    if (b == 0)
        printf("0\n");
    return 0;
}