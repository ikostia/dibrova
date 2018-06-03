from __future__ import print_function
from __future__ import absolute_import

import sys
import datetime
import random

measured_funcs = set()

def measure(func):
    """A decorator to measure total function runtime and call count
    
    Example:
    
    ... @measure
    ... def f():
    ...    do_something()
    ... 
    ... for i in xrange(100):
    ...     f()
    ... 
    ... print(f.total_runtime)
    ... print(f.total_calls)
    """
    if func not in measured_funcs:
        measured_funcs.add(func)
    if not hasattr(func, 'total_runtime'):
        func.total_runtime = 0.0
    if not hasattr(func, 'total_calls'):
        func.total_calls = 0

    def wrapper(*args, **kwargs):
        before_call = datetime.datetime.now()
        res = func(*args, **kwargs)
        elapsed = datetime.datetime.now() - before_call
        func.total_runtime += elapsed.total_seconds()
        func.total_calls += 1
        return res

    return wrapper

def clear_measurements(funcs=None):
    """Clear measurements done by the @measure decorator"""
    if funcs is None:
        funcs = measured_funcs
    for f in funcs:
        f.total_runtime = 0.0
        f.total_calls = 0

def print_func_measuremetns():
    """Pretty-print all the measurements done by the @measure decorator"""
    print("Measured functions:")
    for func in measured_funcs:
        fn = func.func_name
        tr = func.total_runtime
        tc = func.total_calls
        tpc = 'N/A' if tc == 0 else "{:10.10f}".format(tr / tc)
        line = "{:>30}: {:10.8f}s over {:10d} calls ({} per call)".format(fn, tr, tc, tpc)
        print(line)
