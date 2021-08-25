#! /bin/python3
import sys
import kernel
import middle
import json_tools

logPath = "./analyzer_results/"

def analyze_sysinfo(userSide, kernelSide, raw_flag):
    kfunc = kernelSide[0]
    ufunc = userSide[0]

    # syscall info doesn't match between kernel and user level
    # if kfunc != ufunc:
    #     print ("[ERROR] Syscall list in kernel side cannot match to middleware side...")
    #     return 0, 0

    # parameters in user part
    uin = userSide[1]
    uout = userSide[3]

    # parameters in kernel part
    kin = kernelSide[1]
    kout = kernelSide[2]

    # compare input in user and kernel parts
    if not raw_flag:
        results_in, err1 = compare_paras(uin, kin, 1)
        results_out, err2 = compare_paras(uout, kout, 0)
        log_results(kfunc + "-input-paras.info", results_in)
        log_results(kfunc + "-output-paras.info", results_out)
        
        return 0, 0
    else:
        status_in = 0
        status_out = 0

        results_in = compare_paras_raw(uin, kin)
        results_out = compare_paras_raw(uout, kout)
        log_raw_results(kfunc + "-input-paras.raw", results_in)
        log_raw_results(kfunc + "-output-paras.raw", results_out)

        if len(results_in) == 0:
            status_in = 1
        else:
            status_in = 0

        if len(results_out) == 0:
            status_out = 1
        else:
            status_out = 0

        return status_in, status_out


# kpara and upara is the list
paras_chset_in = {}
def compare_paras(upara, kpara, flag):
    global paras_chset_in
    changedSet = {}
    errinfo = "" 
    if len(kpara) != len(upara):
        errinfo = "[ERR] user and kernel part have a different number of parameters..."
        return changedSet, errinfo

    for i in range(len(upara)):
        # init
        if i not in paras_chset_in and flag:
            paras_chset_in[i] = 0

        if str(upara[i]) != str(kpara[i]):
            changedSet[str(i)] = [str(upara[i]), str(kpara[i])]
            info = "[CHG] parameters-" + str(i) + " changed.\n"
            errinfo += info
        elif flag:
            # count uchanged round
            paras_chset_in[i] += 1

    return changedSet, errinfo

def compare_paras_raw(upara, kpara):
    diff = []
    
    # compare init or after parameters
    try:
        diff = json_tools.diff(upara, kpara) 
    except:
        print("[ERR] compare_paras_raw cannot handle non-json...")

    return diff

def log_results(filename, results):
    path = logPath + filename
    with open(path, "a+") as log:
        log.write("\n********** test stage ********\n")
        
        # don't have difference
        if len(results) == 0:
            log.write("All parameters are same.\n")
            return
        
        for item in results:
            info = "loc: " + str(item) + ", user part: " + results[item][0] + ", kernel part: " + results[item][1] + "\n"
            log.write(info)

def log_raw_results(filename, results):
    path = logPath + filename
    with open(path, "a+") as log:
        log.write("\n********** test stage ********\n")

        # don't have difference
        if len(results) == 0:
            log.write("All parameters are same.\n")
            return

        for result in results:
            info = str(result) + "\n"
            log.write(info)

def main():
    # parse and struct the syscall parameters' info
    userSide, raw_userSide = middle.get_sysinfo_middle(sys.argv[1])
    kernelSide, raw_kernelSide = kernel.get_sysinfo_kernel(sys.argv[2])
    all_round = int(sys.argv[3])
    
    # start to analyze
    # if len(kernelSide) != len(userSide):
    #     print ("[ERR in main] the number of syscall logs donnot match in usr and kernel part...")
    #     return
    
    # count the ratio of unchanged parameters
    total_round = 0
    same_round_in = 0
    same_round_out = 0

    # FIX: close(), munmap() double execution issues
    round_num = len(userSide)
    if (all_round * 2) == round_num or all_round == round_num:
        total_round = round_num
    else:
        print ("[WARN] the number of test round doesn't match with log!")
        return

    for i in range(total_round):
        _, _ = analyze_sysinfo(userSide[i], kernelSide[i], False)
        status_in, status_out = analyze_sysinfo(raw_userSide[i], raw_kernelSide[i], True)

        same_round_in += status_in
        same_round_out += status_out

    # all set of parameters ratio
    global paras_chset_in
    print ("[INFO] input parameters unchanged ratio: ", same_round_in/total_round)  
    
    # each parameters' ratio
    for item in paras_chset_in:
        print ("[INFO] parameters: -" + str(item) + "- unchanged ratio: ", paras_chset_in[item]/total_round)

if __name__ == "__main__":
    main()
