#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <linux/mman.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <sys/mman.h>

#include <assert.h>

//WL: mremap 25, msync, 26, mincore 27, madvise 28
#include <unistd.h>
#include <sys/syscall.h>
// #include "marker.h"
// #include "globalv.h"

// #include <sys/types.h>
// #include <sys/mman.h>
// int madvise(caddr_t addr, size_t len, int advice);
// madvise() 函数提供了以下标志，这些标志影响 lgroup 之间线程内存的分配方式：

// MADV_ACCESS_DEFAULT
// 此标志将指定范围的内核预期访问模式重置为缺省设置。
// MADV_ACCESS_LWP
// 此标志通知内核，移近指定地址范围的下一个 LWP 就是将要访问此范围次数最多的 LWP。内核将相应地为此范围和 LWP 分配内存和其他资源。
// MADV_ACCESS_MANY
// 此标志建议内核，许多进程或 LWP 将在系统内随机访问指定的地址范围。内核将相应地为此范围分配内存和其他资源。

// madvise() 函数可以返回以下值：
// EAGAIN
// 指定地址范围（从 addr 到 addr+len）中的部分或所有映射均已锁定进行 I/O 操作。
// EINVAL
// addr 参数的值不是sysconf(3C) 返回的页面大小的倍数，指定地址范围的长度小于或等于零或者建议无效。
// EIO
// 读写文件系统时发生 I/O 错误。
// ENOMEM
// 指定地址范围中的地址不在进程的有效地址空间范围内，或者指定地址范围中的地址指定了一个或多个未映射的页面。
// ESTALE
// NFS 文件句柄过时。


// static void get_rss(const char *tag)
// {
//     char buf[128];
//     snprintf(buf, sizeof(buf), "ps hu %d | cut -c-80", getpid());
//     printf("%-25s ==> ", tag);
//     fflush(stdout);
//     system(buf);
// }

#define CHECK(thing)    \
    if (!(thing))       \
    {                   \
        perror(#thing); \
        exit(1);        \
    }

#define MAX_PAGE_IN 104857600
#define MIN(a, b) ((a) < (b) ? (a) : (b))

//检查指针int有效指针
int valid_pointer(void *p, size_t len)
{
    //获取页面大小并计算页面掩码
    size_t pagesz = sysconf(_SC_PAGESIZE);
    size_t pagemask = ~(pagesz - 1);
    //计算基址
    void *base = (void *)(((size_t)p) & pagemask);
    int result;
    result = msync(base, len, MS_ASYNC);
    // syscall_wrapper("msync", result, SYS_msync, base, len, MS_ASYNC)
    int rv = ( result == 0);
    return rv;
}

int main()
{
    // module_log("memadvance");

    void *s, *x;
    x = malloc(8192);

    x = (unsigned int)x + 0x1000;
    x = (unsigned int)x & 0xfffff000;

    //WL: function not implemented in the OS
    s = (void *)mremap(x, 4000, 8, 0);
    // syscall_wrapper("mremap", s, SYS_mremap, x, 4000, 8, 0)

    perror("mremap");
    printf("old %p new %p\n", x, s);
    //WL: don't free x or s
    return 0;
}
