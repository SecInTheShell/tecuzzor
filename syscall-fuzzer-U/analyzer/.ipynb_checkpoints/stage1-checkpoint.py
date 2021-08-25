#!/usr/bin/python3

import argparse
import pathlib

# for colored cli printing, refer: https://stackoverflow.com/questions/287871/how-to-print-colored-text-to-the-terminal?page=1&tab=votes#tab-top
class bcolors:
    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKCYAN = '\033[96m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'    

class Syscall:

    """
    Syscall class used as a struct to hold info derived from stratce

    self.rv: return value
    self.syscall: syscall function name, or None if the syscall is not captured by strace
    self.args: a list of arguments in str passed to the syscall
    """
    def __init__(self, line: str):
        """initialize the 

        Args:
            line (str): line in the log

        Returns:
            Syscall: a initialized syscall
        """
        # print(line)
        syscall = line.split(' = ')
        assert(len(syscall) == 2)
        self.rv = syscall[1].strip()
        
        syscall[0] = syscall[0].strip()
        deliPos = syscall[0].index('(')
        self.syscall = syscall[0][:deliPos]
        
        args = syscall[0][deliPos + 1: -1].split(',')
        self.args = [x.strip() for x in args]


def parseLog(log: str) -> list:
    """parse the log from a log file

    Args:
        log (str): path to the log file

    Returns:
        list: a list of Syscalls expected to be served by the OS
    """

    syscalls = []
    # set this flag if our notation has been detected, which means the next line should be the expected syscall
    detectFlag = False
    captured = None

    with open(log) as log_raw:
        for line in log_raw:
            if detectFlag and (not "----" in line):
                # print("detecting")
                captured = Syscall(line)
                detectFlag = False
            # elif detectFlag:
            #     syscalls.append(None)
            #     detectFlag = False

            if "testing syscall" in line and "----" in line and line[:5] == "write":
                if " begin " in line:
                    detectFlag = True
                    # print("set detect")
                
                if " end " in line:
                    detectFlag = False
                    syscalls.append(captured)
                    captured = None
                    # print("clear detect")
        
#     print(syscalls)
    return syscalls


def main():

    # argparsing
    parser = argparse.ArgumentParser(description='Differencial analysis to the log files of strace')
    parser.add_argument('raw_log', type=pathlib.Path, 
                        help='raw syscall strace log file')
    parser.add_argument('rt_log', type=pathlib.Path, 
                        help='shielding runtime syscall strace log file')

    args = parser.parse_args()

    rawLog = args.raw_log
    rtLog = args.rt_log

    # process logs
    print("Handling raw log...")
    rawCalls = parseLog(rawLog)

    # for call in rawCalls:
    #     if call:
    #         print(call.syscall, call.args, call.rv)

    print("Handling runtime log...")
    grapheneCalls = parseLog(rtLog)

    # for call in grapheneCalls:
    #     if call:
    #         print(call.syscall, call.args, call.rv)

    exposedSyscalls = set()

    # print(len(rawCalls))
    # print(len(grapheneCalls))

    assert(len(rawCalls) == len(grapheneCalls))
    for i in range(len(rawCalls)):
        print(bcolors.OKCYAN , i, "th syscall", bcolors.ENDC, end="")
        print(" | Original syscall: ", rawCalls[i].syscall)
        print("                ", end="")

        # DEBUG
        # print(rawCalls[i])
        # print(grapheneCalls[i])

        if not grapheneCalls[i]:
            print(bcolors.FAIL, "NOT SERVED!", bcolors.ENDC)
        elif grapheneCalls[i].syscall ==  rawCalls[i].syscall:
            print(bcolors.OKGREEN, "served as", grapheneCalls[i].syscall, bcolors.ENDC)
            exposedSyscalls.add(rawCalls[i].syscall)
        else:
            print(bcolors.WARNING , "but served as", grapheneCalls[i].syscall, bcolors.ENDC)
            exposedSyscalls.add(grapheneCalls[i].syscall)

    print("Exposed syscalls: ", exposedSyscalls)

if __name__ == "__main__":
    main()