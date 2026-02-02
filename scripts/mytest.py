#!/usr/bin/env python

from invoke import run

ret = run("echo from invoke ins script")

print(ret.stdout)  # should print "from invoke\n"
