#! /bin/python3

# using to parse the dmesg syscall info
import os
import sys
import json

def get_sysinfo_middle(filename):
    # read output from the middleware
    # sysinfos is a LISTs contains func and parameters' map
    # format: [func_str, input_list, return_list, output_list]
    sysinfos = []
    with open(filename, "r") as log:
        lines = log.readlines()
        for i in range(len(lines)-2):
            #try:
            if "---- calling" in lines[i] and "---- result" in lines[i+1] and "---- after" in lines[i+2]:
                func1, init = parse_middle_log(lines[i])
                func2, ret = parse_middle_log(lines[i+1])
                func3, after = parse_middle_log(lines[i+2])
                
                # judge if these info are from same syscall
                if func1 == func2 and func2 == func3:
                    sysinfo = [func1, init, ret, after]
                    sysinfos.append(sysinfo)
                    print (sysinfo)
                else:
                    print ("[ERR] Log format is error!")
                    break
            #except:
            #    print ("error")
   
    return sysinfos

def parse_middle_log(msg):
    # get syscall function and body
    func = "null"
    sysinfo = []
    if "---- calling" in msg or "---- after" in msg:
        func = msg.split("::")[3].split(" ")[0].strip(":")
        body = msg.split("::")[3].split(" ", 1)[1].strip()

        infos = json.loads(body)
        if infos == None:
            return func, sysinfo
        for item in infos:
            if type(infos[item]).__name__ == 'dict':
                values = infos[item]
                for key in values:
                    sysinfo.append(values[key])
            elif type(infos[item]).__name__ == 'list':
                for data in infos[item]:
                    if type(data).__name__ == 'dict':
                        for key in data:
                            sysinfo.append(data[key])
                            print (type(data[key]))
                    else:
                        sysinfo.append(data)
            else:
                sysinfo.append(infos[item])

    elif "---- result" in msg:
        func = msg.split("::")[3].split(" ")[0].strip()
        body = msg.split(":")[-1].strip()

        sysinfo.append(body.split("(")[0])
        sysinfo.append(body.split("(")[1].replace(")", ""))

    return func, sysinfo

# get_sysinfo_middle("test1.txt")