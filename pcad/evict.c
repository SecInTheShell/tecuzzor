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
#define PIDMAX 32786
#define handle_error(msg) \
    do { perror(msg); exit(EXIT_FAILURE); } while (0)

//char targetfileName[] = "/home/lys/Documents/pagecachefiles/targetfile%d";
char targetfileName[] = "/home/liuweijie/Documents/pagecachefiles/targetfile%d";
//char evictfileName[] = "/home/lys/Documents/pagecachefiles/evictfile%dG_%d";
char evictfileName[] = "/home/liuweijie/Documents/pagecachefiles/evictfile%dG_%d";
//char lockfileName[] = "/home/lys/Documents/pagecachefiles/lockfile20G";
char lockfileName[] = "/home/liuweijie/Documents/pagecachefiles/lockfile";

int control = 0;

inline __attribute__((always_inline)) void maccess(volatile void* p)
{
  asm volatile ("movq (%0), %%rax\n"
    :
    : "c" (p)
    : "rax");
}

int get_pid_list(int pids[]){
    DIR *d;
    int count = 0;
    struct dirent *dir;
    d = opendir("/proc/");
    if (d) {
        while ((dir = readdir(d)) != NULL) {
            char* pdir = dir->d_name;
            int pid = atoi(pdir);
            if (pid){
                pids[count++] = pid;
            }
        }
        closedir(d);
    }
    return count;
}

int lsof(int pid, char libs[1024][1035]){
    int count = 0;
    FILE *fp;
    char path[1035];

    char cmd[50];
    sprintf(cmd, "lsof -w -p %d | awk '{print $9}' | grep '\\.so'", pid);
    fp = popen(cmd, "r");
    if (fp == NULL) {
        printf("Failed to run command\n" );
        exit(1);
    }

    while (fgets(path, sizeof(path), fp) != NULL) {
        // for (int i=0; i<sizeof(path); i++){
        //     libs[count][i] = path[i];
        // }
        strcpy(libs[count], path);
        libs[count][strlen(libs[count]) - 1] = 0;
        count ++;
    }
    pclose(fp);
    return count;
}

unsigned long file_size(int fd){
    return (unsigned long)lseek(fd, 0, SEEK_END);
}

unsigned long filelist_size(char files[1024][1035], int lc){
    unsigned long lsize = 0;
    for (int j=0; j<lc; j++){
        int  fd = open(files[j] , O_RDONLY); 
        if (fd == -1){
            continue;
            // handle_error("open");
        }
        unsigned long f_size = file_size(fd);
        lsize += f_size;
        close(fd);
    }
    return lsize;
}

unsigned long get_filesize_p(int pid){
    char libs[1024][1035];
    unsigned long p_size = 0;
    int libc = lsof(pid, libs);
    if (libc){
        return filelist_size(libs, libc);
    }
    return 0;
}

int check_file_exist(char allibs[1024][1035], int allibc, char* file){
    for (int i=0; i<allibc; i++){
        if (strcmp(allibs[i], file) == 0){
            return 1;
        }
    }
    return 0;
}

int get_all_files(int pids[], int pc, char allibs[1024][1035]){
    int allibc = 0;
    for (int i=0; i<pc; i++){
        char libs[1024][1035];
        int libc = lsof(pids[i], libs);
        if (libc){
            for (int j=0; j<libc; j++){
                if (!check_file_exist(allibs, allibc, libs[j])){
                    strcpy(allibs[allibc], libs[j]);
                    allibc ++;
                }
            }
        }
    }
    return allibc;
}

void get_set1(char allibs[1024][1035], int allibc, void* set1[], int lib_size[]){
    for (int i=0; i<allibc; i++){
        int  fd = open(allibs[i] , O_RDONLY); 
        int size = (int)lseek(fd, 0, SEEK_END);
        lib_size[i] = size;
        set1[i] = mmap(NULL, size, PROT_READ, MAP_SHARED, fd, 0);
    }
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

void traverse_set1(void* set1[], int set1_size, int lib_size[]){

    for (int i=0; i<set1_size; i++){
        traverse(set1[i], lib_size[i]);
    }
}

void set1_func(){
    int pids[PIDMAX];
    int pc = get_pid_list(pids);
    printf("get proccess list.\n");
    char allibs[1024][1035];
    int allibc = get_all_files(pids, pc, allibs);
    printf("get all files mapped.\n");
    void* set1[allibc];
    int lib_size[allibc];
    get_set1(allibs, allibc, set1, lib_size);
    printf("set1 generated.\n");
    printf("Travesrsing set1...\n");
    while (1){
        traverse_set1(set1, allibc, lib_size);
        sched_yield();
        sched_yield();
        sched_yield();
    }
}

void traverse_rand(int x, int no){
    char dir[100];
    sprintf(dir, evictfileName, x, no);
    int fd = open(dir, O_RDONLY); 
    if (fd == -1)
    {
        printf("%s loading failed\n", dir);
    }
    else
    {
        printf("%s founded\n", dir);
    }
    s_t size = (s_t)lseek(fd, 0, SEEK_END);
    s_t pc = size / (4096);
    void* addr = mmap(NULL, size, PROT_READ, MAP_SHARED, fd, 0);
    traverse(addr, size);

    close(fd);
    munmap(addr, size);
}

void* lock4G(){
    int fd = open("/home/nice/Desktop/set_files/lockfile4G", O_RDONLY); 
    s_t size = (s_t)lseek(fd, 0, SEEK_END);
    s_t pc = size / (4096);
    void* addr = mmap(NULL, size, PROT_READ, MAP_SHARED, fd, 0);
    traverse(addr, size);
    mlock(addr, size);
    return addr;
}

void print_page_state(unsigned char v[], int vl){
    int count = 0;
    float percent = 0;
    for (int i=0; i<vl; i++){
        if (v[i] == 0)
        {
            count++;
        }
    }
    if (count == 0)
        printf("none page has been eviced\n");
    else if (count == vl)
    {
        printf("all pages have been eviced\n");
    }
    else
    {
        percent = (float)count/vl;
        printf("%f has been evicted\n", percent);
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

void print_targetfile_state(char targetfileName[], s_t size_target, void* addr_target)
{
    int pc_target = size_target / (4096);
    unsigned char v[pc_target];
    printf("%s state:\n", targetfileName);
    mincore(addr_target, size_target, v);
    print_page_state(v, pc_target);
}

void set2_func(){
    sched_yield();

    char targetfileName1[100];
    sprintf(targetfileName1, targetfileName, 1);
    char targetfileName2[100];
    sprintf(targetfileName2, targetfileName, 2);
    char targetfileName3[100];
    sprintf(targetfileName3, targetfileName, 3);

    s_t size_target1;
    void* addr_target1;
    get_targetfile(targetfileName1, &size_target1, &addr_target1);
    print_targetfile_state(targetfileName1, size_target1, addr_target1);

    s_t size_target2;
    void* addr_target2;
    get_targetfile(targetfileName2, &size_target2, &addr_target2);
    print_targetfile_state(targetfileName2, size_target2, addr_target2);

    s_t size_target3;
    void* addr_target3;
    get_targetfile(targetfileName3, &size_target3, &addr_target3);
    print_targetfile_state(targetfileName3, size_target3, addr_target3);
    
    int fd_lock = open(lockfileName, O_RDONLY);
    if (fd_lock == -1)
    {
        printf("%s loading failed\n", lockfileName);
    }
    else
    {
        printf("lock file founded.\n");
    }
    s_t size_lock = (s_t)lseek(fd_lock, 0, SEEK_END);
    s_t pc_lock = size_lock / (4096);
    void* addr_lock = mmap(NULL, size_lock, PROT_READ, MAP_SHARED, fd_lock, 0);
    printf("traversing lockfile...\n");
    traverse(addr_lock, size_lock);
    mlock(addr_lock, size_lock);
    printf("lockfile locked.\n");
    print_targetfile_state(targetfileName1, size_target1, addr_target1);
    print_targetfile_state(targetfileName2, size_target2, addr_target2);
    print_targetfile_state(targetfileName3, size_target3, addr_target3);

    int evivt_num = 8;
    for (int i=0; i<evivt_num; i++){
        traverse_rand(1, i);
        printf("evictfile%d traversed.\n", i);
        print_targetfile_state(targetfileName1, size_target1, addr_target1);
        print_targetfile_state(targetfileName2, size_target2, addr_target2);
        print_targetfile_state(targetfileName3, size_target3, addr_target3);
    }

    munmap(addr_target1, size_target1);
    munmap(addr_target2, size_target2);
    munmap(addr_target3, size_target3);

    close(fd_lock);
    munmap(addr_lock, size_lock);
    exit(0);
}

int main(){

    pthread_attr_t attr;
    pthread_attr_init(&attr);
    pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);

    void* set1_retval;
    pthread_t set1_thread;
    int set1_ret = pthread_create(&set1_thread, &attr, (void *)&set1_func, (void*)"set1");
    if (set1_ret != 0){
		printf("??\n");
	}
    
///////////////////////////////////
    void* set2_retval;
    pthread_t set2_thread;
    int set2_ret = pthread_create(&set2_thread, &attr, (void *)&set2_func, (void*)"set2");
    if (set2_ret != 0){
		printf("??\n");
	}

///////////////////////////////////

    pthread_exit(NULL);
    return 0;
}
