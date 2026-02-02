#!/usr/bin/env python3

from invoke import run

ret = run("echo from invoke")

print(ret.stdout)  # should print "from invoke\n"
