#! /usr/bin/python3

# using to parse the dmesg syscall info returned by kernel
import os
import sys
import json

def get_sysinfo_kernel(filename):
    # read output from the kernel
    # sysinfos is a LISTs contains func and parameters' map
    # format: [func_str, input_list, return_list, output_list]
    sysinfos = []
    raw_sysinfos = []
    
    with open(filename, "r") as log:
        func = ""
        sysinfo_in = []
        sysinfo_in_raw = {}      # format: json
        sysinfo_out = []
        sysinfo_out_raw = {}     # format: json

        lines = log.readlines()
        for i in range(len(lines)-2):
            # FIX: special issue: fd = 1
            fd_ignore_flag = 0
            
            # TODO: [hooking] in same line
            # TODO: [hooking] doesn't in the same line
            if "[hooking]" in lines[i] and "type:input" in lines[i]:
                try:
                    func = lines[i+1].strip().split(" ")[-1].split(":")[1].strip()
                    sysinfo_in, sysinfo_in_raw = parse_kernel_log(lines[i+2])
                    
                    # FIX: special issue: fd = 1
                    if "\"fd\":1" in lines[i+2]:
                        fd_ignore_flag = 1
                except:
                    print("[ERR-K-PART:input] Parsing sysinfo from dmesg failed...", lines[i+1], lines[i+2])
                    return []

            elif "[hooking]" in lines[i] and "type:output" in lines[i]:
                try:
                    func_tmp = lines[i+1].strip().split(" ")[-1].split(":")[1].strip()
                    sysinfo_out, sysinfo_out_raw = parse_kernel_log(lines[i+2])

                    # FIX: special issue: fd = 1
                    if "\"fd\":1" in lines[i+2]:
                        fd_ignore_flag = 1
                except:
                    print("[ERR-K-PART:output] Parsing sysinfo from dmesg failed...", lines[i+1], lines[i+2])
                    return []

                if func_tmp == func and not fd_ignore_flag:
                    # parsed sysinfo
                    sysinfo = [func, sysinfo_in, sysinfo_out]
                    sysinfos.append(sysinfo)

                    # raw sysinfo
                    sysinfo = [func, sysinfo_in_raw, sysinfo_out_raw]
                    raw_sysinfos.append(sysinfo)

    return sysinfos, raw_sysinfos

def parse_kernel_log(msg):
    sysinfo = []
    raw_sysinfo = {}
    if "{" in msg and "}" in msg:
        body = msg.split(" ")[-1].strip()

        infos = json.loads(body)
        raw_sysinfo = infos
        if infos == None:
            return func, sysinfo
        for item in infos:
            if type(infos[item]).__name__ == 'dict':
                values = infos[item]
                for key in values:
                    sysinfo.append(values[key])
            elif type(infos[item]).__name__ == 'list':
                flag = 0
                for data in infos[item]:
                    if type(data).__name__ == 'dict':
                        for key in data:
                            sysinfo.append(data[key])
                            # print (type(data[key]))
                    else:
                        flag = 1
                        break
                if flag == 1 or len(infos[item]) == 0:
                    sysinfo.append(infos[item])
            else:
                sysinfo.append(infos[item])

    return sysinfo, raw_sysinfo

#sysinfo, raw = get_sysinfo_kernel("../dmesg.txt")