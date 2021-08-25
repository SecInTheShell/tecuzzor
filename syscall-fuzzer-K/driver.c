#define pr_fmt(fmt) "ftrace_hook: " fmt

#include <linux/ftrace.h>
#include <linux/kallsyms.h>
#include <linux/kernel.h>
#include <linux/linkage.h>
#include <linux/module.h>
#include <linux/slab.h>
#include <linux/uaccess.h>
#include <linux/version.h>
#include <linux/sched.h>

#include <linux/string.h>

// debugfs
#include <linux/debugfs.h>

#include <linux/moduleparam.h>

int hook_syscall_no = -1;
module_param(hook_syscall_no, int, S_IRUSR | S_IWUSR);

// #define STAGE3

// for rust
extern void syscall_logger(int, unsigned long, unsigned long, unsigned long, unsigned long, unsigned long, unsigned long);
extern void syscall_logger_input(int, unsigned long, unsigned long, unsigned long, unsigned long, unsigned long, unsigned long);
extern void syscall_logger_output(int, unsigned long, unsigned long, unsigned long, unsigned long, unsigned long, unsigned long);

pid_t target_pid;

#define LEN_DBG_PIDS_BUF 1024
static char dbg_pids_buf[LEN_DBG_PIDS_BUF];

static ssize_t nr_chrs(const char *str, ssize_t len_str, const char chr)
{
    ssize_t ret, i;

    ret = 0;
    for (i = 0; i < len_str; i++)
    {
        if (str[i] == '\0')
            break;
        if (str[i] == chr)
            ret++;
    }

    return ret;
}

static const char *next_chr(const char *str, ssize_t len, const char chr)
{
    ssize_t i;

    for (i = 0; i < len; i++)
    {
        if (str[0] == '\0')
            break;
        if (str[0] == chr)
            break;
        str++;
    }

    return str;
}

static unsigned long *str_to_ints(const char *str, ssize_t len,
                                  ssize_t *nr_integers)
{
    unsigned long *list;
    ssize_t i;

    if (str == NULL || len == 0 || (str[0] < '0' || str[0] > '9'))
    {
        *nr_integers = 0;
        return NULL;
    }
    *nr_integers = nr_chrs(str, len, ' ') + 1;
    list = kmalloc_array(*nr_integers, sizeof(unsigned long), GFP_KERNEL);

    for (i = 0; i < *nr_integers; i++)
    {
        if (sscanf(str, "%lu ", &list[i]) != 1)
            break;
        str = next_chr(str, len, ' ') + 1;
    }

    return list;
}

static ssize_t debugfs_pids_write(struct file *file,
                                  const char __user *buf, size_t count, loff_t *ppos)
{
    ssize_t ret;
    unsigned long *targets;
    ssize_t nr_targets;

    ret = simple_write_to_buffer(dbg_pids_buf, LEN_DBG_PIDS_BUF,
                                 ppos, buf, count);
    if (ret < 0)
        return ret;

    targets = str_to_ints(dbg_pids_buf, ret, &nr_targets);
    target_pid = (pid_t)(*targets);
    printk("[hooking-pid] pid is %d\n", target_pid);

    kfree(targets);

    return ret;
}

static const struct file_operations pids_fops = {
    .owner = THIS_MODULE,
    .write = debugfs_pids_write,
};

static struct dentry *debugfs_root;

static int __init debugfs_init(void)
{
    // init target pid
    target_pid = 0;

    debugfs_root = debugfs_create_dir("middleware", NULL);
    if (!debugfs_root)
    {
        pr_err("[hooking-debugfs] failed to create debugfs\n");
        return -ENOMEM;
    }

    if (!debugfs_create_file("pids", 0600, debugfs_root, NULL, &pids_fops))
    {
        pr_err("[hooking-debugfs] failed to create pids file\n");
        return -ENOMEM;
    }

    return 0;
}

static void __exit debugfs_exit(void)
{
    debugfs_remove_recursive(debugfs_root);
}

//------------------- hooking syscall ---------------------
MODULE_DESCRIPTION("module hooking execve() via ftrace");
MODULE_AUTHOR("WL/HC");
MODULE_LICENSE("GPL");

/*
  * 挂接时有两种防止恶性递归循环的方法：
  * 使用函数返回地址（USE_FENTRY_OFFSET = 0）检测响应
  * 通过跳过ftrace调用来避免回避（USE_FENTRY_OFFSET = 1）
  */
#define USE_FENTRY_OFFSET 0

/**
  * struct ftrace_hook-描述要安装的单个钩子
  *
  * name：要挂接的函数的名称
  * @function：指向要执行的函数的指针
  * @original：指向保存指针的位置的指针
  * 
  * 恢复原始功能
  * @address：函数条目的内核地址
  * @ops：此函数挂钩的ftrace_ops状态
  *
  * 用户只能填写＆name，＆hook，＆orig字段。
  * 其他字段视为实施细节。
  */
struct ftrace_hook
{
    const char *name;
    void *function;
    void *original;

    unsigned long address;
    struct ftrace_ops ops;
};

static int fh_resolve_hook_address(struct ftrace_hook *hook)
{
    hook->address = kallsyms_lookup_name(hook->name);

    if (!hook->address)
    {
        pr_debug("unresolved symbol: %s\n", hook->name);
        return -ENOENT;
    }

#if USE_FENTRY_OFFSET
    *((unsigned long *)hook->original) = hook->address + MCOUNT_INSN_SIZE;
#else
    *((unsigned long *)hook->original) = hook->address;
#endif

    return 0;
}

static void notrace fh_ftrace_thunk(unsigned long ip, unsigned long parent_ip,
                                    struct ftrace_ops *ops, struct pt_regs *regs)
{
    struct ftrace_hook *hook = container_of(ops, struct ftrace_hook, ops);

#if USE_FENTRY_OFFSET
    regs->ip = (unsigned long)hook->function;
#else
    if (!within_module(parent_ip, THIS_MODULE))
        regs->ip = (unsigned long)hook->function;
#endif
}

/**
  * fh_install_hooks（）-注册并启用一个钩子
  * @hook：要安装的钩子
  *
  * 返回：成功则返回零，否则返回负错误代码。
  */
int fh_install_hook(struct ftrace_hook *hook)
{
    int err;
    //备份原地址
    err = fh_resolve_hook_address(hook);
    if (err)
        return err;

    /*
     * 修改％rip寄存器，因此需要IPMODIFY标志
     * 并以SAVE_REGS为前提。 ftrace的抗递归防护
     * 如果更改％rip无效，使用RECURSION_SAFE将其禁用。
     * 将执行自己的跟踪功能重新输入检查。
     */
    hook->ops.func = fh_ftrace_thunk;
    hook->ops.flags = FTRACE_OPS_FL_SAVE_REGS | FTRACE_OPS_FL_RECURSION_SAFE | FTRACE_OPS_FL_IPMODIFY;

    err = ftrace_set_filter_ip(&hook->ops, hook->address, 0, 0);
    if (err)
    {
        pr_debug("ftrace_set_filter_ip() failed: %d\n", err);
        return err;
    }

    err = register_ftrace_function(&hook->ops);
    if (err)
    {
        pr_debug("register_ftrace_function() failed: %d\n", err);
        ftrace_set_filter_ip(&hook->ops, hook->address, 1, 0);
        return err;
    }

    return 0;
}

/**
 * fh_remove_hooks() - 禁用和注销一个钩子
 * hook: 被注销的hook结构
 */
void fh_remove_hook(struct ftrace_hook *hook)
{
    int err;

    err = unregister_ftrace_function(&hook->ops);
    if (err)
    {
        pr_debug("unregister_ftrace_function() failed: %d\n", err);
    }

    err = ftrace_set_filter_ip(&hook->ops, hook->address, 1, 0);
    if (err)
    {
        pr_debug("ftrace_set_filter_ip() failed: %d\n", err);
    }
}

/**
  * fh_install_hooks（）-注册并启用多个挂钩
  * hooks：要安装的钩子结构体数组
  * count：要安装的挂钩数量
  *
  * 整个挂钩过程必须一次完成，如果某些钩子函数无法安装，则所有挂钩将被删除。
  *
  * 返回：成功则返回零，否则返回负错误代码。
  */
int fh_install_hooks(struct ftrace_hook *hooks, size_t count)
{
    int err;
    size_t i;

    for (i = 0; i < count; i++)
    {
        err = fh_install_hook(&hooks[i]);
        if (err)
            goto error;
    }

    return 0;

error:
    while (i != 0)
    {
        fh_remove_hook(&hooks[--i]);
    }

    return err;
}

/**
  * fh_remove_hooks（）-禁用和注销多个钩子
  * hooks：要删除的钩子数组
  * count：要删除的挂钩数
  */
void fh_remove_hooks(struct ftrace_hook *hooks, size_t count)
{
    size_t i;

    for (i = 0; i < count; i++)
        fh_remove_hook(&hooks[i]);
}

#ifndef CONFIG_X86_64
#error Currently only x86_64 architecture is supported
#endif

#if defined(CONFIG_X86_64) && (LINUX_VERSION_CODE >= KERNEL_VERSION(4, 17, 0))
#define PTREGS_SYSCALL_STUBS 1
#endif

/*
  *尾部调用优化可能会干扰基于堆栈上返回地址的递归检测。禁用以避免机器挂断。
  */
#if !USE_FENTRY_OFFSET
#pragma GCC optimize("-fno-optimize-sibling-calls")
#endif

static char *duplicate_filename(const char __user *filename)
{
    char *kernel_filename;

    kernel_filename = kmalloc(4096, GFP_KERNEL);
    if (!kernel_filename)
        return NULL;

    if (strncpy_from_user(kernel_filename, filename, 4096) < 0)
    {
        kfree(kernel_filename);
        return NULL;
    }

    return kernel_filename;
}

/*#ifdef PTREGS_SYSCALL_STUBS
static asmlinkage long (*real_sys_execve)(struct pt_regs *regs);

static asmlinkage long fh_sys_execve(struct pt_regs *regs)
{
    long ret;
    char *kernel_filename;

    kernel_filename = duplicate_filename((void*) regs->di);

    pr_info("execve() before: %s\n", kernel_filename);

    kfree(kernel_filename);

    ret = real_sys_execve(regs);

    pr_info("execve() after: %ld\n", ret);

    return ret;
}
#else
static asmlinkage long (*real_sys_execve)(
        const char __user *filename,
        const char __user *const __user *argv,
        const char __user *const __user *envp);

static asmlinkage long fh_sys_execve(
        const char __user *filename,
        const char __user *const __user *argv,
        const char __user *const __user *envp)
{
    char *kernel_filename;
    long ret;
    kernel_filename = duplicate_filename(filename);
    //printk("%s start", kernel_filename);
    ret = real_sys_execve(filename, argv, envp);
    //printk("%s end", kernel_filename);
    kfree(kernel_filename);
    return ret;
}
#endif
*/

/*
 * fuzzing syscall list
 * https://github.com/ya0guang/syscall-fuzzer-Rust#syscall-support
 */

bool check_target_process(pid_t pid)
{
    if (target_pid == 0)
    {
        // printk("[hooking-status] target pid is not set, stop hooking...");
        return false;
    }

    if (pid == target_pid)
    {
        printk("[hooking-status] target pid match, start hooking");
        return true;
    }
    else
        return false;
}

//open: ZL: strange
static asmlinkage long (*real_sys_open)(
    const char __user *filename,
    int flags, umode_t mode);

static asmlinkage long fh_sys_open(
    const char __user *filename,
    int flags, umode_t mode)
{
    long ret;
    char *kernel_filename;

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
    {
        ret = real_sys_open(filename, flags, mode);
        return ret;
    }

    // printk parameters: input
    kernel_filename = duplicate_filename(filename);
    syscall_logger_input(2, kernel_filename, flags, mode, 0, 0, 0);

    // do real syscall
    ret = real_sys_open(filename, flags, mode);

    // printk parameters: output
    kernel_filename = duplicate_filename(filename);
    syscall_logger_output(2, kernel_filename, flags, mode, 0, 0, 0);

    return ret;
}

//close
static asmlinkage long (*real_sys_close)(
    unsigned int fd);

static asmlinkage long fh_sys_close(
    unsigned int fd)
{
    long ret;

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
    {
        ret = real_sys_close(fd);
        return ret;
    }

    // printk parameters: input
    syscall_logger_input(3, fd, 0, 0, 0, 0, 0);

    // do real syscall
    ret = real_sys_close(fd);

    // printk parameters: output
    syscall_logger_output(3, fd, 0, 0, 0, 0, 0);

    return ret;
}

// read
static asmlinkage long (*real_sys_read)(
    unsigned int fd, char __user *buf, size_t count);

static asmlinkage long fh_sys_read(
    unsigned int fd, char __user *buf, size_t count)
{
    long ret;
    char *kernel_buf;

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
    {
        ret = real_sys_read(fd, buf, count);
        return ret;
    }

    // printk parameters: input
    kernel_buf = kmalloc(count, GFP_KERNEL);
    copy_from_user(kernel_buf, buf, count);
    syscall_logger_input(0, fd, kernel_buf, count, 0, 0, 0);

    // do real syscall
    ret = real_sys_read(fd, buf, count);

    // printk parameters: output
    copy_from_user(kernel_buf, buf, count);
    syscall_logger_output(0, fd, kernel_buf, count, 0, 0, 0);

    kfree(kernel_buf);
    return ret;
}

// write
static asmlinkage long (*real_sys_write)(
    unsigned int fd, const char __user *buf, size_t count);

static asmlinkage long fh_sys_write(
    unsigned int fd, const char __user *buf, size_t count)
{
    long ret;
    char *kernel_buf;

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
    {
        ret = real_sys_write(fd, buf, count);
        return ret;
    }

    // printk parameters: input
    kernel_buf = kmalloc(count, GFP_KERNEL);
    copy_from_user(kernel_buf, buf, count);
    syscall_logger_input(1, fd, kernel_buf, count, 0, 0, 0);

    // do real syscall
    ret = real_sys_write(fd, buf, count);

    // printk parameters: output
    copy_from_user(kernel_buf, buf, count);
    syscall_logger_output(1, fd, kernel_buf, count, 0, 0, 0);
    // syscall_logger(1, fd, kernel_buf, count, 0, 0, 0);

    kfree(kernel_buf);
    return ret;
}

// stat
static asmlinkage long (*real_sys_newstat)(
    const char __user *filename,
    struct stat __user *statbuf);

static asmlinkage long fh_sys_newstat(
    const char __user *filename,
    struct stat __user *statbuf)
{
    long ret;
    char *kernel_filename;
    struct stat *kstatbuf;

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
    {
        ret = real_sys_newstat(filename, statbuf);
        return ret;
    }

    // printk parameters: input
    kstatbuf = kmalloc(sizeof(struct stat), GFP_KERNEL);
    kernel_filename = duplicate_filename(filename);
    copy_from_user(kstatbuf, statbuf, sizeof(kstatbuf));
    syscall_logger_input(4, kernel_filename, kstatbuf, 0, 0, 0, 0);

    // do real syscall
    ret = real_sys_newstat(filename, statbuf);

    // printk parameters: output
    kernel_filename = duplicate_filename(filename);
    copy_from_user(kstatbuf, statbuf, sizeof(kstatbuf));
    syscall_logger_output(4, kernel_filename, kstatbuf, 0, 0, 0, 0);

    kfree(kstatbuf);
    return ret;
}

// fstat
static asmlinkage long (*real_sys_newfstat)(
    unsigned int fd,
    struct stat __user *statbuf);

static asmlinkage long fh_sys_newfstat(
    unsigned int fd,
    struct stat __user *statbuf)
{
    long ret;
    struct stat *kstatbuf;

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
    {
        ret = real_sys_newfstat(fd, statbuf);
        return ret;
    }

    // printk input
    kstatbuf = kmalloc(sizeof(struct stat), GFP_KERNEL);
    copy_from_user(kstatbuf, statbuf, sizeof(kstatbuf));
    syscall_logger_input(5, fd, kstatbuf, 0, 0, 0, 0);

    //WL: debugging
    // printk("GFP_KERNEL value: 0x%lx\n", GFP_KERNEL);

    // do real syscall
    ret = real_sys_newfstat(fd, statbuf);

    // printk output
    copy_from_user(kstatbuf, statbuf, sizeof(kstatbuf));
    syscall_logger_output(5, fd, kstatbuf, 0, 0, 0, 0);

    kfree(kstatbuf);
    return ret;
}

// lstat:
static asmlinkage long (*real_sys_newlstat)(
    const char __user *filename,
    struct stat __user *statbuf);

static asmlinkage long fh_sys_newlstat(
    const char __user *filename,
    struct stat __user *statbuf)
{
    long ret;
    char *kernel_filename;
    struct stat *kstatbuf;

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
    {
        ret = real_sys_newlstat(filename, statbuf);
        return ret;
    }

    // printk parameters: input
    kernel_filename = duplicate_filename(filename);
    kstatbuf = kmalloc(sizeof(struct stat), GFP_KERNEL);
    copy_from_user(kstatbuf, statbuf, sizeof(kstatbuf));

    syscall_logger_input(6, kernel_filename, kstatbuf, 0, 0, 0, 0);

    // do real syscall
    ret = real_sys_newlstat(filename, statbuf);

    kernel_filename = duplicate_filename(filename);
    copy_from_user(kstatbuf, statbuf, sizeof(kstatbuf));

    syscall_logger_output(6, kernel_filename, kstatbuf, 0, 0, 0, 0);

    kfree(kstatbuf);
    return ret;
}

// poll

struct kernel_pollfd
{
    int fd;
    short events;
    short revents;
};

static asmlinkage long (*real_sys_poll)(
    struct pollfd __user *ufds, unsigned int nfds, int timeout);

static asmlinkage long fh_sys_poll(
    struct pollfd __user *ufds, unsigned int nfds, int timeout)
{
    long ret;

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
    {
        ret = real_sys_poll(ufds, nfds, timeout);
        return ret;
    }

    struct kernel_pollfd *kernel_ufds;
    kernel_ufds = kmalloc(sizeof(struct kernel_pollfd) * nfds, GFP_KERNEL);
    copy_from_user(kernel_ufds, ufds, sizeof(struct kernel_pollfd) * nfds);

    // printk("[hooking] func: Poll, type: init, ");
    // int i;
    // for (i = 0; i < nfds; i++)
    // {
    //     int pollfd_fd = kernel_ufds[i].fd;
    //     short pollfd_events = kernel_ufds[i].events;
    //     short pollfd_revents = kernel_ufds[i].revents;
    //     printk("fd: %d, events: %d, revents: %d, ", pollfd_fd, pollfd_events, pollfd_revents);
    // }
    // printk("nfds: %u, timeout: %d\n", nfds, timeout);
    syscall_logger_input(7, kernel_ufds, nfds, timeout, 0, 0, 0);

    // do real syscall
    ret = real_sys_poll(ufds, nfds, timeout);

    copy_from_user(kernel_ufds, ufds, sizeof(struct kernel_pollfd) * nfds);
    // printk("[hooking] func: Poll, type: after, ret: %ld, ", ret);
    // for (i = 0; i < nfds; i++)
    // {
    //     int pollfd_fd = kernel_ufds[i].fd;
    //     short pollfd_events = kernel_ufds[i].events;
    //     short pollfd_revents = kernel_ufds[i].revents;
    //     printk("fd: %d, events: %d, revents: %d, ", pollfd_fd, pollfd_events, pollfd_revents);
    // }
    // printk("nfds: %u, timeout: %d\n", nfds, timeout);
    syscall_logger_output(7, kernel_ufds, nfds, timeout, 0, 0, 0);

    return ret;
}

//lseek
static asmlinkage long (*real_sys_lseek)(
    unsigned int fd, off_t offset,
    unsigned int whence);

static asmlinkage long fh_sys_lseek(
    unsigned int fd, off_t offset,
    unsigned int whence)
{
    long ret;

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
    {
        ret = real_sys_lseek(fd, offset, whence);
        return ret;
    }

    // input
    syscall_logger_input(8, fd, offset, whence, 0, 0, 0);

    // do real syscall
    ret = real_sys_lseek(fd, offset, whence);

    // output
    syscall_logger_output(8, fd, offset, whence, 0, 0, 0);

    return ret;
}

// pread64
static asmlinkage long (*real_sys_pread64)(
    unsigned int fd, char __user *buf,
    size_t count, loff_t pos);

static asmlinkage long fh_sys_pread64(
    unsigned int fd, char __user *buf,
    size_t count, loff_t pos)
{
    long ret;
    char *kernel_buf;

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
    {
        ret = real_sys_pread64(fd, buf, count, pos);
        return ret;
    }

    // printk parameters: input
    kernel_buf = kmalloc(count, GFP_KERNEL);
    copy_from_user(kernel_buf, buf, count);
    syscall_logger_input(17, fd, kernel_buf, count, pos, 0, 0);

    // do real syscall
    ret = real_sys_pread64(fd, buf, count, pos);

    // printk parameters: output
    copy_from_user(kernel_buf, buf, count);
    syscall_logger_output(17, fd, kernel_buf, count, pos, 0, 0);

    kfree(kernel_buf);

    return ret;
}

// pwrite64
static asmlinkage long (*real_sys_pwrite64)(
    unsigned int fd, const char __user *buf,
    size_t count, loff_t pos);

static asmlinkage long fh_sys_pwrite64(
    unsigned int fd, const char __user *buf,
    size_t count, loff_t pos)
{
    long ret;
    char *kernel_buf;

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
    {
        ret = real_sys_pwrite64(fd, buf, count, pos);
        return ret;
    }

    // printk parameters: input
    kernel_buf = kmalloc(count, GFP_KERNEL);
    copy_from_user(kernel_buf, buf, count);
    syscall_logger_input(18, fd, kernel_buf, count, pos, 0, 0);

    // do real syscall
    ret = real_sys_pwrite64(fd, buf, count, pos);

    // printk parameters: output
    copy_from_user(kernel_buf, buf, count);
    syscall_logger_output(18, fd, kernel_buf, count, pos, 0, 0);

    kfree(kernel_buf);

    return ret;
}

struct iovec
{
    void *iov_base; /* Starting address */
    size_t iov_len; /* Number of bytes to transfer */
};

// writev
static asmlinkage long (*real_sys_writev)(
    unsigned long fd,
    const struct iovec __user *vec,
    unsigned long vlen);

static asmlinkage long fh_sys_writev(
    unsigned long fd,
    const struct iovec __user *vec,
    unsigned long vlen)
{
    long ret;
    struct iovec *kernel_vec;
    struct iovec *kernel_vec_ori;
    int i;

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
    {
        ret = real_sys_writev(fd, vec, vlen);
        return ret;
    }

    // 16 is the size of struct iovec
    kernel_vec = kmalloc(vlen * 16, GFP_KERNEL);
    copy_from_user(kernel_vec, vec, vlen * 16);

    kernel_vec_ori = kmalloc(vlen * 16, GFP_KERNEL);
    copy_from_user(kernel_vec_ori, vec, vlen * 16);
    // get iov_base in each kernel_vec[i]
    for (i = 0; i < vlen; i++)
    {
        kernel_vec[i].iov_base = kmalloc((kernel_vec_ori[i]).iov_len, GFP_KERNEL);
        copy_from_user(kernel_vec[i].iov_base, (kernel_vec_ori[i]).iov_base, (kernel_vec_ori[i]).iov_len);
    }

    syscall_logger_input(20, fd, kernel_vec, vlen, 0, 0, 0);

    ret = real_sys_writev(fd, vec, vlen);

    for (i = 0; i < vlen; i++)
    {
        copy_from_user(kernel_vec[i].iov_base, (kernel_vec_ori[i]).iov_base, (kernel_vec_ori[i]).iov_len);
    }
    syscall_logger_output(20, fd, kernel_vec, vlen, 0, 0, 0);

    // printk("[hooking] func: writev, type: input, fd: %ld, vlen: %ld\n", fd, vlen);
    for (i = 0; i < vlen; i++)
    {
        kfree((kernel_vec[i]).iov_base);
    }
    kfree(kernel_vec);
    kfree(kernel_vec_ori);

    return ret;
}

//readv: fuzzing uncompleted
static asmlinkage long (*real_sys_readv)(
    unsigned long fd,
    const struct iovec __user *vec,
    unsigned long vlen);

static asmlinkage long fh_sys_readv(
    unsigned long fd,
    const struct iovec __user *vec,
    unsigned long vlen)
{
    long ret;
    struct iovec *kernel_vec;
    struct iovec *kernel_vec_ori;
    int i;

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
    {
        ret = real_sys_readv(fd, vec, vlen);
        return ret;
    }

    // 16 is the size of struct iovec
    kernel_vec = kmalloc(vlen * 16, GFP_KERNEL);
    copy_from_user(kernel_vec, vec, vlen * 16);

    kernel_vec_ori = kmalloc(vlen * 16, GFP_KERNEL);
    copy_from_user(kernel_vec_ori, vec, vlen * 16);
    // get iov_base in each kernel_vec[i]
    for (i = 0; i < vlen; i++)
    {
        kernel_vec[i].iov_base = kmalloc((kernel_vec_ori[i]).iov_len, GFP_KERNEL);
        copy_from_user(kernel_vec[i].iov_base, (kernel_vec_ori[i]).iov_base, (kernel_vec_ori[i]).iov_len);
    }

    syscall_logger_input(19, fd, kernel_vec, vlen, 0, 0, 0);
    // do real syscall
    ret = real_sys_readv(fd, vec, vlen);

    for (i = 0; i < vlen; i++)
    {
        copy_from_user(kernel_vec[i].iov_base, (kernel_vec_ori[i]).iov_base, (kernel_vec_ori[i]).iov_len);
    }
    syscall_logger_output(19, fd, kernel_vec, vlen, 0, 0, 0);

    // printk("[hooking] func: readv, type: input, fd: %ld, vlen: %ld\n", fd, vlen);

    for (i = 0; i < vlen; i++)
    {
        kfree((kernel_vec[i]).iov_base);
    }
    kfree(kernel_vec);
    kfree(kernel_vec_ori);

    return ret;
}

// access
static asmlinkage long (*real_sys_access)(
    const char __user *filename, int mode);

static asmlinkage long fh_sys_access(
    const char __user *filename, int mode)
{
    long ret;
    char *kernel_filename;

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
    {
        ret = real_sys_access(filename, mode);
        return ret;
    }

    // printk parameters: input
    kernel_filename = duplicate_filename(filename);
    syscall_logger_input(21, kernel_filename, mode, 0, 0, 0, 0);

    // do real syscall
    ret = real_sys_access(filename, mode);

    // printk parameters: output
    kernel_filename = duplicate_filename(filename);
    syscall_logger_output(21, kernel_filename, mode, 0, 0, 0, 0);

    return ret;
}

// pipe
static asmlinkage long (*real_sys_pipe)(
    int __user *fildes);

static asmlinkage long fh_sys_pipe(
    int __user *fildes)
{
    long ret;
    int *kernel_fildes;

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
    {
        ret = real_sys_pipe(fildes);
        return ret;
    }

    //WL: current, pipe takes 2 int
    kernel_fildes = kmalloc(2 * sizeof(int), GFP_KERNEL);
    copy_from_user(kernel_fildes, fildes, 2 * sizeof(int));
    syscall_logger_input(22, kernel_fildes, 0, 0, 0, 0, 0);

    ret = real_sys_pipe(fildes);

    copy_from_user(kernel_fildes, fildes, 2 * sizeof(int));
    syscall_logger_output(22, kernel_fildes, 0, 0, 0, 0, 0);

    kfree(kernel_fildes);

    return ret;
}

// pipe2
static asmlinkage long (*real_sys_pipe2)(
    int __user *fildes, int flags);

static asmlinkage long fh_sys_pipe2(
    int __user *fildes, int flags)
{
    long ret;
    int *kernel_fildes;

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
    {
        ret = real_sys_pipe2(fildes, flags);
        return ret;
    }

    //WL: count is 2, usually
    kernel_fildes = kmalloc(2 * sizeof(int), GFP_KERNEL);
    copy_from_user(kernel_fildes, fildes, 2 * sizeof(int));
    // printk("[hooking] func: pipe, type: input, *fildes: %p, fildes: %d, flags: %d\n", fildes, kernel_fildes, flags);
    syscall_logger_input(293, kernel_fildes, flags, 0, 0, 0, 0);

    ret = real_sys_pipe2(fildes, flags);

    copy_from_user(kernel_fildes, fildes, 2 * sizeof(int));
    syscall_logger_output(293, kernel_fildes, flags, 0, 0, 0, 0);

    return ret;
}

// select: fuzzing uncompleted, output uncompleted
static asmlinkage long (*real_sys_select)(
    int n, fd_set __user *inp, fd_set __user *outp,
    fd_set __user *exp, struct timeval __user *tvp);

static asmlinkage long fh_sys_select(
    int n, fd_set __user *inp, fd_set __user *outp,
    fd_set __user *exp, struct timeval __user *tvp)
{
    long ret;
    ret = real_sys_select(n, inp, outp, exp, tvp);

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
        return ret;

    printk("[hooking] func: select, type: input, n: %d, *inp: %p, *outp: %p, *exp: %p, *tvp: %p", n, inp, outp, exp, tvp);

    return ret;
}

// mmap
static asmlinkage long (*real_sys_mmap_pgoff)(
    unsigned long addr, unsigned long len,
    unsigned long prot, unsigned long flags,
    unsigned long fd, off_t pgoff);

static asmlinkage long fh_sys_mmap_pgoff(
    unsigned long addr, unsigned long len,
    unsigned long prot, unsigned long flags,
    unsigned long fd, off_t pgoff)
{
    long ret;

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
    {
        ret = real_sys_mmap_pgoff(addr, len, prot, flags, fd, pgoff);
        return ret;
    }

    // input
    syscall_logger_input(9, addr, len, prot, flags, fd, pgoff);

    // do real syscall
    ret = real_sys_mmap_pgoff(addr, len, prot, flags, fd, pgoff);

    // output
    syscall_logger_output(9, addr, len, prot, flags, fd, pgoff);

    return ret;
}

// munmap
static asmlinkage long (*real_sys_munmap)(
    unsigned long addr, size_t len);

static asmlinkage long fh_sys_munmap(
    unsigned long addr, size_t len)
{
    long ret;

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
    {
        ret = real_sys_munmap(addr, len);
        return ret;
    }

    // input
    syscall_logger_input(11, addr, len, 0, 0, 0, 0);

    // do real syscall
    ret = real_sys_munmap(addr, len);

    // output
    syscall_logger_output(11, addr, len, 0, 0, 0, 0);

    return ret;
}

// mprotect
static asmlinkage long (*real_sys_mprotect)(
    unsigned long start, size_t len,
    unsigned long prot);

static asmlinkage long fh_sys_mprotect(
    unsigned long start, size_t len,
    unsigned long prot)
{
    long ret;

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
    {
        ret = real_sys_mprotect(start, len, prot);
        return ret;
    }

    // input
    syscall_logger_input(10, start, len, prot, 0, 0, 0);

    // do real syscall
    ret = real_sys_mprotect(start, len, prot);

    // output
    syscall_logger_output(10, start, len, prot, 0, 0, 0);

    return ret;
}

// brk
static asmlinkage long (*real_sys_brk)(
    unsigned long addr);

static asmlinkage long fh_sys_brk(
    unsigned long addr)
{
    long ret;

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
    {
        ret = real_sys_brk(addr);
        return ret;
    }

    // input
    syscall_logger_input(12, addr, 0, 0, 0, 0, 0);

    // do real syscall
    ret = real_sys_brk(addr);

    // output
    syscall_logger_output(12, addr, 0, 0, 0, 0, 0);

    return ret;
}

// mremap
//WL: cannot execute mremap on user-space
static asmlinkage long (*real_sys_mremap)(
    unsigned long addr,
    unsigned long old_len, unsigned long new_len,
    unsigned long flags, unsigned long new_addr);

static asmlinkage long fh_sys_mremap(
    unsigned long addr,
    unsigned long old_len, unsigned long new_len,
    unsigned long flags, unsigned long new_addr)
{
    long ret;

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
    {
        ret = real_sys_mremap(addr, old_len, new_len, flags, new_addr);
        return ret;
    }

    // input
    syscall_logger_input(25, addr, old_len, new_len, flags, new_addr, 0);

    // do real syscall
    ret = real_sys_mremap(addr, old_len, new_len, flags, new_addr);

    // output
    syscall_logger_output(25, addr, old_len, new_len, flags, new_addr, 0);

    return ret;
}

// getuid
static asmlinkage long (*real_sys_getuid)(void);

static asmlinkage long fh_sys_getuid(void)
{
    long ret;
    ret = real_sys_getuid();

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
        return ret;

    printk("[hooking] func: Getuid, type: output, ret: %ld\n", ret);

    return ret;
}

// geteuid
static asmlinkage long (*real_sys_geteuid)(void);

static asmlinkage long fh_sys_geteuid(void)
{
    long ret;
    ret = real_sys_geteuid();

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
        return ret;

    printk("[hooking] func: Geteuid, type: output, ret: %ld\n", ret);

    return ret;
}

// gettid
static asmlinkage long (*real_sys_gettid)(void);

static asmlinkage long fh_sys_gettid(void)
{
    long ret;
    ret = real_sys_gettid();

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
        return ret;

    printk("[hooking] func: Gettid, type: output, ret: %ld\n", ret);

    return ret;
}

// getpid
static asmlinkage long (*real_sys_getpid)(void);

static asmlinkage long fh_sys_getpid(void)
{
    long ret;
    ret = real_sys_getpid();

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
        return ret;

    printk("[hooking] func: getpid, type: output, ret: %ld\n", ret);

    return ret;
}

// getppid
static asmlinkage long (*real_sys_getppid)(void);

static asmlinkage long fh_sys_getppid(void)
{
    long ret;
    ret = real_sys_getppid();

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
        return ret;

    printk("[hooking] func: Getppid, type: output, ret: %ld\n", ret);

    return ret;
}

//nanosleep (for kernel 4.15)
static asmlinkage long (*real_sys_nanosleep)(
    struct timespec __user *rqtp,
    struct timespec __user *rmtp);

static asmlinkage long fh_sys_nanosleep(
    struct timespec __user *rqtp,
    struct timespec __user *rmtp)
{
    long ret = 0;

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
    {
        ret = real_sys_nanosleep(rqtp, rmtp);
        return ret;
    }

    struct timespec kernel_rqtp;
    struct timespec kernel_rmtp;

    copy_from_user(&kernel_rqtp, rqtp, sizeof(struct timespec));
    copy_from_user(&kernel_rmtp, rmtp, sizeof(struct timespec));

    syscall_logger_input(35, &kernel_rqtp, &kernel_rmtp, 0, 0, 0, 0);
    // printk("[hooking] func: nanosleep, type: intput, rqtp.tv_sec: %ld, rmtp.tv_sec: %ld\n", kernel_rqtp.tv_sec, kernel_rmtp.tv_sec);

#ifndef STAGE3
    ret = real_sys_nanosleep(rqtp, rmtp);
#endif

    copy_from_user(&kernel_rqtp, rqtp, sizeof(struct timespec));
    copy_from_user(&kernel_rmtp, rmtp, sizeof(struct timespec));

    syscall_logger_output(35, &kernel_rqtp, &kernel_rmtp, 0, 0, 0, 0);

    return ret;
}

/* for kernel 5.4 */
//nanosleep:
/*static asmlinkage long (*real_sys_nanosleep)(
        struct __kernel_timespec __user *rqtp,
        struct __kernel_timespec __user *rmtp);

static asmlinkage long fh_sys_nanosleep(
        struct __kernel_timespec __user *rqtp,
        struct __kernel_timespec __user *rmtp)
{
    long ret;
    printk("[lizhi] nanosleep start\n");
    ret = real_sys_nanosleep(rqtp, rmtp);
    printk("[lizhi] nanosleep end\n");

    return ret;
}*/

// socket
static asmlinkage long (*real_sys_socket)(int domain, int type, int protocol);

static asmlinkage long fh_sys_socket(int domain, int type, int protocol)
{
    long ret;

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
    {
        ret = real_sys_socket(domain, type, protocol);
        return ret;
    }

    // input:
    syscall_logger_input(41, domain, type, protocol, 0, 0, 0);

    // do real syscall
    ret = real_sys_socket(domain, type, protocol);

    // output
    syscall_logger_output(41, domain, type, protocol, 0, 0, 0);

    // printk("[hooking] func: socket, type: input, unknown1: %d, unknown2: %d, unknown3: %d\n", pm1, pm2, pm3);

    return ret;
}

// connect
struct kernel_sockaddr
{
    short sa_family;
    char sa_data[14];
};

static asmlinkage long (*real_sys_connect)(int pm1, struct kernel_sockaddr __user *pm2, int pm3);

static asmlinkage long fh_sys_connect(int pm1, struct kernel_sockaddr __user *pm2, int pm3)
{
    long ret;
    struct kernel_sockaddr kernel_addr;

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
    {
        ret = real_sys_connect(pm1, pm2, pm3);
        return ret;
    }

    copy_from_user(&kernel_addr, pm2, sizeof(struct kernel_sockaddr));
    syscall_logger_input(42, pm1, &kernel_addr, pm3, 0, 0, 0);

    // do real syscall
    ret = real_sys_connect(pm1, pm2, pm3);

    copy_from_user(&kernel_addr, pm2, sizeof(struct kernel_sockaddr));
    // short kernel_sa_family = kernel_addr.sa_family;
    // char *kernel_sa_data = &(kernel_addr.sa_data[14]);
    // printk("[hooking] func: connect, type: input, unknown1: %d, *sockaddr: %p, kernel_sa_family: %d, kernel_sa_data: %s, unknown3: %d\n", pm1, pm2, kernel_sa_family, kernel_sa_data, pm3);
    syscall_logger_output(42, pm1, &kernel_addr, pm3, 0, 0, 0);

    return ret;
}

// send: sys_send is not correct
/*static asmlinkage long (*real_sys_send) (int socket, void __user *msg, size_t len, unsigned flags);

static asmlinkage long fh_sys_send (int socket, void __user *msg, size_t len, unsigned flags)
{
    long ret;
    ret = real_sys_send(socket, msg, len, flags);

    // copy from userspace
    char *kernel_msg;
    kernel_msg = kmalloc(len, GFP_KERNEL);
    copy_from_user(kernel_msg, msg, sizeof(char) * len);

    printk("[hooking] func: send, type: input, socket: %d, *msg: %p, msg: %s, len: %ld, flags: %u\n", socket, msg, kernel_msg, len, flags);

    return ret;
}*/

//send/sendto
static asmlinkage long (*real_sys_sendto)(
    int s, void __user *msg, size_t len, unsigned flags,
    struct kernel_sockaddr __user *socket, int n);

static asmlinkage long fh_sys_sendto(
    int s, void __user *msg, size_t len, unsigned flags,
    struct kernel_sockaddr __user *socket, int n)
{
    long ret;
    ret = real_sys_sendto(s, msg, len, flags, socket, n);

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
    {
        ret = real_sys_sendto(s, msg, len, flags, socket, n);
        return ret;
    }

    // copy from userspace
    char *kernel_msg;
    kernel_msg = kmalloc(len, GFP_KERNEL);
    copy_from_user(kernel_msg, msg, sizeof(char) * len);

    struct kernel_sockaddr kernel_socket;
    copy_from_user(&kernel_socket, socket, sizeof(kernel_socket));

    syscall_logger_input(44, s, kernel_msg, len, flags, &kernel_socket, n);

    // do real syscall
    ret = real_sys_sendto(s, msg, len, flags, socket, n);

    // short kernel_sa_family = kernel_socket.sa_family;
    // char *kernel_sa_data = &(kernel_socket.sa_data[14]);
    // printk("[hooking] func: send, type: input, socket: %d, *msg: %p, msg: %s, len: %ld, flags: %u, *sockaddr: %p, kernel_sa_family: %d, kernel_sa_data: %s, n: %d\n", s, msg, kernel_msg, len, flags, socket, kernel_sa_family, kernel_sa_data, n);

    copy_from_user(kernel_msg, msg, sizeof(char) * len);
    copy_from_user(&kernel_socket, socket, sizeof(kernel_socket));

    syscall_logger_output(44, s, kernel_msg, len, flags, &kernel_socket, n);

    kfree(kernel_msg);

    return ret;
}

// recvfrom
static asmlinkage long (*real_sys_recvfrom)(
    int s, void __user *msg, size_t len, unsigned flags,
    struct kernel_sockaddr __user *socket, int n);

static asmlinkage long fh_sys_recvfrom(
    int s, void __user *msg, size_t len, unsigned flags,
    struct kernel_sockaddr __user *socket, int n)
{
    long ret;

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
    {
        ret = real_sys_recvfrom(s, msg, len, flags, socket, n);
        return ret;
    }

    // copy from userspace
    // char *kernel_msg;
    // kernel_msg = kmalloc(len, GFP_KERNEL);
    // copy_from_user(kernel_msg, msg, sizeof(char) * len);

    // struct kernel_sockaddr kernel_socket;
    //copy_from_user(&kernel_socket, socket, sizeof(kernel_socket));
    // copy_from_user(&kernel_socket, socket, n);

    // short kernel_sa_family = kernel_socket.sa_family;
    // char *kernel_sa_data = &(kernel_socket.sa_data[14]);
    // printk("[hooking] func: RecvFrom, type: init, socketfd: %d, len: %ld, flags: %u\n", s, len, flags);

    // syscall_logger_input(45, s, kernel_msg, len, flags, &kernel_socket, n);

    //do real syscall
    ret = real_sys_recvfrom(s, msg, len, flags, socket, n);

    // copy_from_user(kernel_msg, msg, sizeof(char) * len);
    // copy_from_user(&kernel_socket, socket, n);
    // kernel_sa_family = kernel_socket.sa_family;
    // *kernel_sa_data = &(kernel_socket.sa_data[14]);
    // printk("[hooking] func: RecvFrom, type: after, ret: %ld, socketfd: %d, len: %ld, flags: %u\n", ret, s, len, flags);

    // syscall_logger_output(45, s, kernel_msg, len, flags, &kernel_socket, n);

    // kfree(kernel_msg);
    return ret;
}

// getsockopt
static asmlinkage long (*real_sys_getsockopt)(
    int fd, int level, int optname,
    char __user *optval, int __user *optlen);

static asmlinkage long fh_sys_getsockopt(
    int fd, int level, int optname,
    char __user *optval, int __user *optlen)
{
    long ret;
    char *kernel_optval;
    int kernel_optlen;

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
    {
        ret = real_sys_getsockopt(fd, level, optname, optval, optlen);
        return ret;
    }

    // copy from userspace
    copy_from_user(&kernel_optlen, optlen, sizeof(int));

    kernel_optval = kmalloc(kernel_optlen, GFP_KERNEL);
    copy_from_user(kernel_optval, optval, sizeof(char) * kernel_optlen);
    syscall_logger_input(55, fd, level, optname, kernel_optval, &kernel_optval, 0);
    // printk("[hooking] func: Getsockopt, type: init, socket: %d, level: %d, optname: %d, optlen: %d\n", fd, level, optname, kernel_optlen);

    // do real syscall
    ret = real_sys_getsockopt(fd, level, optname, optval, optlen);

    copy_from_user(&kernel_optlen, optlen, sizeof(int));
    copy_from_user(kernel_optval, optval, sizeof(char) * kernel_optlen);
    syscall_logger_output(55, fd, level, optname, kernel_optval, &kernel_optval, 0);

    // printk("[hooking] func: Getsockopt, type: after, ret: %ld, socket: %d, level: %d, optname: %d, optlen: %d\n", ret, fd, level, optname, kernel_optlen);

    kfree(kernel_optval);

    return ret;
}

// setsockopt
static asmlinkage long (*real_sys_setsockopt)(
    int fd, int level, int optname,
    char __user *optval, int optlen);

static asmlinkage long fh_sys_setsockopt(
    int fd, int level, int optname,
    char __user *optval, int optlen)
{
    long ret;

    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
    {
        ret = real_sys_setsockopt(fd, level, optname, optval, optlen);
        return ret;
    }

    // printk parameters
    char *kernel_optval;
    kernel_optval = kmalloc(sizeof(char) * optlen, GFP_KERNEL);

    copy_from_user(kernel_optval, optval, sizeof(char) * optlen);
    syscall_logger_input(54, fd, level, optname, kernel_optval, optlen, 0);
    // printk("[hooking] func: Setsockopt, type: init, socket: %d, level: %d, optname: %d, optlen: %d\n", fd, level, optname, optlen);

    // do real syscall
    ret = real_sys_setsockopt(fd, level, optname, optval, optlen);

    copy_from_user(kernel_optval, optval, sizeof(char) * optlen);
    syscall_logger_output(54, fd, level, optname, kernel_optval, optlen, 0);

    // printk("[hooking] func: Setsockopt, type: after, ret: %ld, socket: %d, level: %d, optname: %d, optlen: %d\n", ret, fd, level, optname, optlen);

    kfree(kernel_optval);

    return ret;
}

/*
 * x86_64 kernels have a special naming convention for syscall entry points in newer kernels.
 * That's what you end up with if an architecture has 3 (three) ABIs for system calls.
 */
#ifdef PTREGS_SYSCALL_STUBS
#define SYSCALL_NAME(name) ("__x64_" name)
#else
#define SYSCALL_NAME(name) (name)
#endif

#define HOOK(_name, _function, _original) \
    {                                     \
        .name = SYSCALL_NAME(_name),      \
        .function = (_function),          \
        .original = (_original),          \
    }

struct ftrace_hook hook_table[] = {
    HOOK("sys_read", fh_sys_read, &real_sys_read),          //check for kernel 4.15
    HOOK("sys_write", fh_sys_write, &real_sys_write),       //check for kernel 4.15
    HOOK("sys_open", fh_sys_open, &real_sys_open),          //check for kernel 4.15
    HOOK("sys_close", fh_sys_close, &real_sys_close),       //check for kernel 4.15
    HOOK("sys_newstat", fh_sys_newstat, &real_sys_newstat), //check for kernel 4.15
    HOOK("sys_newfstat", fh_sys_newfstat, &real_sys_newfstat),
    HOOK("sys_newlstat", fh_sys_newlstat, &real_sys_newlstat),
    HOOK("sys_poll", fh_sys_poll, &real_sys_poll),
    HOOK("sys_lseek", fh_sys_lseek, &real_sys_lseek),                //check for kernel 4.15
    HOOK("sys_mmap_pgoff", fh_sys_mmap_pgoff, &real_sys_mmap_pgoff), //uncompleted
    HOOK("sys_mprotect", fh_sys_mprotect, &real_sys_mprotect),
    HOOK("sys_munmap", fh_sys_munmap, &real_sys_munmap),
    HOOK("sys_brk", fh_sys_brk, &real_sys_brk),
    HOOK("sys_pread64", fh_sys_pread64, &real_sys_pread64),    //check for kernel 4.15
    HOOK("sys_pwrite64", fh_sys_pwrite64, &real_sys_pwrite64), //check for kernel 4.15
    HOOK("sys_readv", fh_sys_readv, &real_sys_readv),
    HOOK("sys_writev", fh_sys_writev, &real_sys_writev),
    HOOK("sys_access", fh_sys_access, &real_sys_access),
    HOOK("sys_pipe", fh_sys_pipe, &real_sys_pipe),
    HOOK("sys_mremap", fh_sys_mremap, &real_sys_mremap), //unchecked
    HOOK("sys_nanosleep", fh_sys_nanosleep, &real_sys_nanosleep),
    HOOK("sys_getpid", fh_sys_getpid, &real_sys_getpid),
    HOOK("sys_socket", fh_sys_socket, &real_sys_socket),
    HOOK("sys_connect", fh_sys_connect, &real_sys_connect),
    HOOK("sys_sendto", fh_sys_sendto, &real_sys_sendto),
    HOOK("sys_recvfrom", fh_sys_recvfrom, &real_sys_recvfrom),
    HOOK("sys_setsockopt", fh_sys_setsockopt, &real_sys_setsockopt),
    HOOK("sys_getsockopt", fh_sys_getsockopt, &real_sys_getsockopt),
    HOOK("sys_getuid", fh_sys_getuid, &real_sys_getuid),
    HOOK("sys_geteuid", fh_sys_geteuid, &real_sys_geteuid),
    HOOK("sys_getppid", fh_sys_getppid, &real_sys_getppid),
    HOOK("sys_gettid", fh_sys_gettid, &real_sys_gettid),
    HOOK("sys_pipe2", fh_sys_pipe2, &real_sys_pipe2),
};

//WL: currently, we hook 1 syscall at a time
#define HOOK_NUM 1
static struct ftrace_hook demo_hooks[HOOK_NUM];

void fillin_demo_hooks(int syscall_no)
{
    switch (syscall_no)
    {
    case 0:
        //read
        demo_hooks[0] = hook_table[0];
        break;
    case 1:
        //write
        demo_hooks[0] = hook_table[1];
        break;
    case 2:
        //open
        demo_hooks[0] = hook_table[2];
        break;
    case 3:
        //close
        demo_hooks[0] = hook_table[3];
        break;
    case 4:
        //stat
        demo_hooks[0] = hook_table[4];
        break;
    case 5:
        //fstat
        demo_hooks[0] = hook_table[5];
        break;
    case 6:
        //lstat
        demo_hooks[0] = hook_table[6];
        break;
    case 7:
        //poll
        demo_hooks[0] = hook_table[7];
        break;
    case 8:
        //lseek
        demo_hooks[0] = hook_table[8];
        break;
    case 9:
        //mmap
        demo_hooks[0] = hook_table[9];
        break;
    case 10:
        //mprotect
        demo_hooks[0] = hook_table[10];
        break;
    case 11:
        //munmap
        demo_hooks[0] = hook_table[11];
        break;
    case 12:
        //brk
        demo_hooks[0] = hook_table[12];
        break;
    case 17:
        //pread64
        demo_hooks[0] = hook_table[13];
        break;
    case 18:
        //pwrite64
        demo_hooks[0] = hook_table[14];
        break;
    case 19:
        //readv
        demo_hooks[0] = hook_table[15];
        break;
    case 20:
        //writev
        demo_hooks[0] = hook_table[16];
        break;
    case 21:
        //access
        demo_hooks[0] = hook_table[17];
        break;
    case 22:
        //pipe
        demo_hooks[0] = hook_table[18];
        break;
    case 25:
        //mremap
        demo_hooks[0] = hook_table[19];
        break;
    case 35:
        //nanosleep
        demo_hooks[0] = hook_table[20];
        break;
    case 39:
        //getpid
        demo_hooks[0] = hook_table[21];
        break;
    case 41:
        //socket
        demo_hooks[0] = hook_table[22];
        break;
    case 42:
        //connect
        demo_hooks[0] = hook_table[23];
        break;
    case 44:
        //sendto
        demo_hooks[0] = hook_table[24];
        break;
    case 45:
        //recvfrom
        demo_hooks[0] = hook_table[25];
        break;
    case 54:
        //setsockopt
        demo_hooks[0] = hook_table[26];
        break;
    case 55:
        //getsockopt
        demo_hooks[0] = hook_table[27];
        break;
    case 102:
        //getuid
        demo_hooks[0] = hook_table[28];
        break;
    case 107:
        //geteuid
        demo_hooks[0] = hook_table[29];
        break;
    case 110:
        //getppid
        demo_hooks[0] = hook_table[30];
        break;
    case 186:
        //gettid
        demo_hooks[0] = hook_table[31];
        break;
    case 293:
        //pipe2
        demo_hooks[0] = hook_table[32];
        break;
    default:
        break;
    }
}

static int fh_init(void)
{
    int err;
    printk("Traversal module is working..\n");

    // debugfs install
    debugfs_init();

    fillin_demo_hooks(hook_syscall_no);

    err = fh_install_hooks(demo_hooks, ARRAY_SIZE(demo_hooks));
    if (err)
        return err;
    pr_info("module loaded\n");
    return 0;
}
module_init(fh_init);

static void fh_exit(void)
{
    // debugfs exit
    debugfs_exit();

    // fillin_demo_hooks(hook_syscall_no);

    fh_remove_hooks(demo_hooks, ARRAY_SIZE(demo_hooks));

    pr_info("module unloaded\n");
}
module_exit(fh_exit);
