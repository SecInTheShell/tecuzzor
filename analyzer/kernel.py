#! /bin/python3

# using to parse the dmesg syscall info
import os
import sys
import fcntl
import errno

BUF_SIZE = 512

# construct syscall info with set
def get_sysinfo_kernel():
    # get dmesg info
    sysinfos = read_dmesg_log()

    # paraSet in funcSet is map
    funcSet = parse_dmesg_log(sysinfos)

    #print (funcSet)
    
    return funcSet

# /dev/kmsg is special, readlines() cannot work
def read_dmesg_log():
    # get file number
    f = open("/dev/kmsg", "r")
    fd = os.dup(f.fileno())
    f.close()

    # read dmesg data
    fcntl.fcntl(fd, fcntl.F_SETFL, os.O_NONBLOCK)
    sysinfos = []
    while True:
        try:
            msg = os.read(fd, 512)
        except OSError as e:
            if e.errno == errno.EAGAIN:
                break
            else:
                raise e

        for data in msg.decode("utf-8").split("\n"):
            if "[hooking]" in data:
                sysinfos.append(data)
   
    return sysinfos

def parse_dmesg_log(msg):
    # all syscall infos
    sysinfos = []
    
    # single syscall info
    sysinfo = []
    for i in range(len(msg)):
        func = "null"
        info = {}
        ret = {}
        data = msg[i].split(";")[1]
        items = data.replace("[hooking] ", "").split(", ")
        for item in items:
            name = item.split(": ")[0]
            value = item.split(": ")[1]
            if name == "func":
                func = value
            elif name == "ret":
                ret["ret"] = value
            elif name != "type":
                info[name] = value

        if "ret: " not in data:
            sysinfo.append(func)
            sysinfo.append(info)
        else:
            sysinfo.append(ret)
            sysinfo.append(info)
            sysinfos.append(sysinfo)
            sysinfo = []

    return sysinfos

get_sysinfo_kernel()
