#include <stdio.h>
#include <sys/mman.h>
#include <fcntl.h>
#include <unistd.h>

#define s_t unsigned long long

//char targetfileName[] = "/home/lys/Documents/pagecachefiles/targetfile%d";
char targetfileName[] = "/home/liuweijie/Documents/pagecachefiles/targetfile%d";

inline __attribute__((always_inline)) void maccess(volatile void* p)
{
  asm volatile ("movq (%0), %%rax\n"
    :
    : "c" (p)
    : "rax");
}

void traverse(void* addr, s_t addl){
    if ((long)addr == 0xffffffffffffffff){
        return;
    }
    for (s_t i=3; i<addl - 3; i++){
        maccess(addr + i - 3);
        maccess(addr + i - 2);
        maccess(addr + i - 1);
        maccess(addr + i - 0);
        maccess(addr + i + 1);
        maccess(addr + i + 2);
        maccess(addr + i + 3);
    }
}

void get_targetfile(char targetfileName[], s_t* size_target, void** addr_target)
{
    int fd_target = open(targetfileName, O_RDONLY);
    if (fd_target == -1)
    {
        printf("%s loading failed\n", targetfileName);
    }
    else
    {
        printf("%s founded\n", targetfileName);
    }
    *size_target = (s_t)lseek(fd_target, 0, SEEK_END);
    int pc_target = *size_target / (4096);
    printf("targetfile info: size %lldB pages %d\n", size_target, pc_target);
    unsigned char v[pc_target];
    *addr_target = mmap(NULL, *size_target, PROT_READ, MAP_SHARED, fd_target, 0);
    return;
}

void load_targetfile(char targetfileName[])
{
    void* addr_target;
    s_t size_target;
    get_targetfile(targetfileName, &size_target, &addr_target);
    traverse(addr_target, size_target);
}

int main(int argc, char* argv[])
{
    if (argc == 1)
    {
        return 0;
    }

    int input = (int)(argv[1][0] - 48);

    char dir[100];
    sprintf(dir, targetfileName, input);
    load_targetfile(dir);
}