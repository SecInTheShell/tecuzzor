#! /bin/python3
import sys
import kernel
import middleware

logPath = "./analyzer_results/"

def analyze_parameters(origin, middle):
    # intersection
    inter = list(set(origin).intersection(set(middle)))

    # difference
    diff = list(set(origin).difference(set(middle)))

    return inter, diff

# TODO
def analyze_sysinfo(kernelSide, middleSide):
    # syscall info doesn't match between kernel and user level
    if len(kernelSide) != len(middleSide):
        # TODO
        print ("[ERROR] Syscall list in kernel side cannot match to middleware side...")
        return

    # walk all syscall infos
    for index in range(len(middleSide)):
        # compare syscall info one by one
        kernel = kernelSide[index]
        midlle = middleSide[index]
        
        # analyze 1: compare if the paras value had changed
        changedSet, err = compare_value(kernel, midlle)
        if err != "" and "[ERR]" not in err:
            func = changedSet["func"]
            log2file(func + ".log", changedSet)
            print (func + " ---> " + err)


# output comparing results to file
def log2file(filename, changedSet):
    path = logPath + filename
    with open(path, "w+") as log:
        func = changedSet["func"]
        
        # log init
        if len(changedSet["init"]) != 0:
            result = func + ", type: init"
            for loc in changedSet["init"]:
                results = result + ", parameters-" + str(loc)
                for item in changedSet["init"]:
                    results = results + ", " + str(item)

                log.write(results + "\n")

        # log ret
        if "ret" in changedSet:
            results = func + ", type: return, " + str(changedSet["ret"][0]) + ", " + str(changedSet["ret"][1]) + "\n"
            log.write(results + "\n")

        # log after
        if len(changedSet["after"]) != 0:
            result = func + ", type: after"
            for loc in changedSet["after"]:
                results = result + ", parameters-" + str(loc)
                for item in changedSet["after"]:
                    results = results + ", " + str(item)

                log.write(results + "\n")


# ksys and msys is 3-tuple: [func-name, init-paras, ret-value, after-paras]
def compare_value(ksys, msys):
    changedSet = {}
    err = ""
    
    # compare func
    if ksys[0] != msys[0]:
        err = "[ERR] Cannot compare different syscall..."
        return changedSet, err

    changedSet["func"] = ksys[0]

    # compare init parameters
    kinit = ksys[1]
    minit = msys[1]
    diff_init = compare_value_paras(kinit, minit)
    changedSet["init"] = diff_init

    # compare ret parameters
    kret = ksys[2]
    mret = msys[2]
    if kret != mret[1]:     # mret is a list: ['Ok', '4']
        changedSet["ret"] = [kret, mret]
        err = "[WARN] Return value is different..."

    # compare after parameters
    kafter = ksys[3]
    mafter = msys[3]
    diff_init = compare_value_paras(kafter, mafter)
    changedSet["after"] = diff_init

    if "init" in changedSet or "after" in changedSet:
        err = "[WARN] syscall has differnet parameters in kernel..."

    return changedSet, err

def compare_value_paras(kpara, mpara):
    # key is diff_loc, value is LIST [middle, kernel]
    diff = {}
    
    # compare init or after parameters
    print ("-----------------------")
    print(kpara)
    for i in range(len(kpara)):
        kinfo = kpara[i]
        minfo = mpara[i]

        # minfo contain the type of parametes' value
        if type(minfo).__name__ == 'int':
            kinfo_t = int(kinfo)
            if kinfo_t != minfo:
                diff[i] = [kinfo_t, minfo]

        elif type(minfo).__name__ == 'list':
            kinfo_t = str2ascii(kinfo)
            if kinfo_t != minfo:
                diff[i] = [kinfo, ascii2str(minfo)]

    return diff

def str2ascii(data):
    data_ascii = []
    data_list = list(data)
    
    for i in data_list:
        data_ascii.append(ord(i))

    return data_ascii

def ascii2str(data_list):
    data_str = ""
    
    for item in data_list:
        data_str += chr(item)
    
    return data_str

def main():
    kernelSide = kernel.get_sysinfo_kernel()
    middleSide = middleware.get_sysinfo_middle(sys.argv[1])

    # start to analyze
    analyze_sysinfo(kernelSide, middleSide)

if __name__ == "__main__":
    main()
