#!/bin/python3
import sys
import maketools.build as build
import maketools.fix as fix
import maketools.update as update

help_text = """\
./make.py:
    build:
        build and package scx_controller module
        sugg: try ./make.sh build --help to get details
    format:
        format codes of scx_controller
    fix:
        fix codes of scx_controller
    update:
        recursive update all depended crates
    help:
        print this help\
"""

try:
    arg = sys.argv[1]
except IndexError:
    exit(-1)

match arg:
    case "help":
        print(help_text)
    case "build":
        build.task(sys.argv[2:])
    case "fix":
        fix.task()
    case "update":
        update.task()
    case _:
        exit(-1)
