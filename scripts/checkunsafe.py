#!/bin/python3

# Utility checking for unsafes in the code that don't have an override calling them safe. Ex:
    # This is reported as unsafe
        # let x = unsafe { std::mem::zeroed() };
    # However this, is not:
        # // SAFE: Safe
        # let x = unsafe { std::mem::zeroed() };

import re
import sys

def usage():
    print(f"{sys.argv[0]} <INPUT_FILE>")

def main():
    safe_comment_regex = re.compile(r"SAFE: ([Ss][Hh][Oo][Uu][Ll][Dd] [Bb][Ee] )*[Ss]+([Aa][Ff][Ee]).*?", re.MULTILINE)
    unsafe_regex       = re.compile(r'^(?!\s*//).*?\bunsafe\b', re.MULTILINE)

    argv = sys.argv
    argc = len(argv)
    argc_i = 1

    if argc == argc_i:
        print("Expected input file. Usage:")
        usage()
        return

    if argv[argc_i] == "-h" or argv[argc_i] == "--help":
        print("Usage:")
        usage()
        return

    file = open(argv[argc_i])
    lines = file.readlines()

    for (i, line) in enumerate(lines):
        is_unsafe = re.search(unsafe_regex, line)

        if is_unsafe and i == 0:
            print(f"{argv[argc_i]}:{i}")
            continue

        last_line_is_comment = re.search(safe_comment_regex, lines[i - 1])
        if is_unsafe and not last_line_is_comment:
            print(f"{argv[argc_i]}:{i + 1}")
            continue

main()
