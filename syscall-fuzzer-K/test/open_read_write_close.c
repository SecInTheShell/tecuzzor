# include <stdio.h>

int main()
{
	int pid = getpid();
    printf("%d\n", pid);
    
    FILE *fp_tmp = NULL;
    fp_tmp = fopen("/sys/kernel/debug/middleware/pids", "w+");
    fprintf(fp_tmp, "%d", pid);
    fclose(fp_tmp);

    printf("staring...\n");
	
	FILE *fp = NULL;
	char buff[255];
	
	// open
	fp = fopen("test.txt", "rw+");
	printf("[stage1]: open file\n");
     	
	// read
	fscanf(fp, "%s", buff);
	printf("[stage1]: read file: %s\n", buff);

	// write
	fprintf(fp, "This is testing for fprintf...\n");
	printf("[stage1]: write file\n");
	
	// close
	fclose(fp);

	return 0;
}
