#include <stdio.h>
#include <time.h>
#include <signal.h>
#include <errno.h>
#include <unistd.h>

void sigfunc (int sig_no)
{
     int temp = 1000;
     while (temp-- > 0)
     ;
}



int msleep (unsigned long milisec, int temp)
{
     struct timespec req = {0}, rem = {0};
     time_t sec = (int)(milisec / 1000);
     milisec = milisec - (sec * 1000);
     req.tv_sec = sec;            /*秒*/
     req.tv_nsec = milisec * 1000000L;    /*纳秒*/
     while (nanosleep (&req, &req) == -1 && errno == EINTR) {
         printf ("测试-%d被信号中断,剩余时间为: %ld秒%ld纳秒\n", temp, req.tv_sec, req.tv_nsec);
         continue;
     }
     return (1);
}


int main()
{
     // struct sigaction sa = {0};
     // sa.sa_handler = &sigfunc;
     // sigaction (SIGINT, &sa, NULL);   //安装信号处理函数

	int pid = getpid();
    printf("%d\n", pid);
    
    FILE *fp = NULL;
    fp = fopen("/sys/kernel/debug/middleware/pids", "w+");
    fprintf(fp, "%d", pid);
    fclose(fp);

    printf("staring...\n");

     unsigned long a = 7;
     int temp = 1;
     printf("%ld \n", sizeof(long int));

     for (;;) {
             printf ("testing-%d\n", temp);
             msleep (a*1000, temp);  //将 nanosleep() 封装在 msleep() 中
             temp++;
     }
     return (1);
}