# Usage
0. create a DIR **pagecacheFilePath**

## Usage for test
1. change **pagecacheFilePath** in **generate_page_cache_file.sh**(line 1), **evict_demo.c**(line 20-25), and  **load_target_file.sh**(line 1) which currently is "/home/lys/Documents/pagecachefiles"
2. make
3. `./generate_page_cache_file.sh`
4. `./load_target_file.sh`
5. `./evict_demo`

## Usage for attact
1. change **pagecacheFilePath** in **generate_page_cache_file.sh**(line 1), **evict.c**(line 20-25), **victim.c**(line 8-9),**detect.c**(line 17-18) which currently is "/home/lys/Documents/pagecachefiles"
2. make
3. `./generate_page_cache_file.sh`
4. `./evict` (if your program cannot evict target files, you can execute it multiple times or execute `echo 1 > /proc/sys/vm/drop_caches`)
5. `./detect.sh`
6. `./victim 1` (or `./victim 2`, `./victim 3`)
7. wait for result of detect.sh，and the result will be the same as the parameter of victim