#!/bin/bash

proj_path=$(cd `dirname $0`; pwd)
echo "We are now in: "$proj_path

# build hook-rust
make clean && make
if [ $? -ne 0 ]
then
	echo "hook-rust make failed!"
	exit
else
	echo "hook-rust make succeed!"
fi

# check & build fuzzer-rust
cd ..
Install_Path=`pwd`
if [ ! -d "./syscall-fuzzer-Rust" ]
then
	echo "No u-part fuzzer! Please clone rust fuzzer at: "$Install_Path" first!"
	exit
else
	echo "Rust fuzzer exists..."
fi
cd syscall-hook-rust

cd ../syscall-fuzzer-Rust && cargo build --release
if [ $? -ne 0 ]
then
	echo "fuzzer-rust make failed!"
	exit
else
	echo "fuzzer-rust make succeed!"
fi
cd ../syscall-hook-rust

sudo dmesg -c

# TODO: We can check the analyzer results before clean it
mkdir -p stage2-logs analyzer_results

# round
round=3

# raw
# mwp=syscall_fuzzer

# ratel
# ./ratel -- ./dbt_test/hello
cp ../syscall-fuzzer-Rust/target/release/syscall_fuzzer /home/ratel/ratel/
mwp=syscall_fuzzer


# lstat(6), readv(19), writev(20) most are the same
# getpit(39), getuid(102), geteuid(107), getppid(110), gettid(186) are no-argument syscalls
# recvfrom(45) would cause network & sudo crash on a poor machine
array=(0)
# successful ones:
# array=(0 1 2 3 4 5 6 7 8 9 10 11 12 17 18 19 20 21 22 35 41 42 44 54 55 293)
# array=(39 102 107 110)

for no in ${array[@]}
do
	sudo rmmod -f syscall_hook
	rm -rf stage2-logs/*

	# install
	echo "Installing syscall hooks..."
	sudo insmod syscall_hook.ko hook_syscall_no=$no

	echo "Calling syscall-fuzzer-Rust and waiting..."
	# using release version
	# sleep 4s by default, then execute 100 rounds
	#(../syscall-fuzzer-Rust/target/release/syscall_fuzzer -n $round -s 100 -p 4 $no >> ./stage2-logs/user.part)&
	# ratel
	cd /home/ratel/ratel
	(./ratel -- syscall_fuzzer -n $round -s 100 -p 6 $no >> $proj_path/stage2-logs/user.part)&
	cd /home/ratel/syscall-hook-rust
	# ratel usually needs 17s to load the fuzzer
	sleep 18s

	# Todo: set pid in other middleware

	echo "Starting to inject debugfs"
	pid=$(ps -ef | grep $mwp | grep -v grep | awk '{print $2}')
	# see if pid is an integer
	echo "pid: "$pid
	if [ ! -n "$pid" ]; then
		echo "Error! Pid is NULL!"
	else
		echo "Pid is not NULL"
	fi
	if [[ $pid =~ ^[0-9]\{1,\}$ ]]
	then 
		echo "Error! Pid is not an integer!"
		exit
	else
		echo "Pid is an integer. Injecting..."
	fi
	sudo bash -c "echo $pid >> /sys/kernel/debug/middleware/pids"

	# 
	while :
	do
		exist=$(ps -ef | grep $mwp | grep -v grep | awk '{print $2}')
		# echo "exist: "$exist
		if [ ! $exist ]; then 
			break
		fi
		echo $mwp" is not finished, continue..."
		sleep 1s
	done

	sudo dmesg -c >> ./stage2-logs/kernel.part

	# TODO: make analyzer more beatiful
	python3 analyzer/analyzer.py ./stage2-logs/user.part ./stage2-logs/kernel.part $round
done

sudo dmesg -c
