from __future__ import print_function
from __future__ import absolute_import

import sys
import datetime
import random
import unittest

import bst as bstmod


def insert_elements(bst, elements):
    for elem in elements:
        bst.insert(elem)


def find_elements(bst, elements):
    for elem in elements:
        bst.find(elem)


def delete_elements(bst, elements):
    for elem in elements:
        bst.delete(bst.find(elem))


def measure(func):
    before_func = datetime.datetime.now()
    func()
    return datetime.datetime.now() - before_func


def benchmark_tree(name, bst, n=1000):
    # if the depth of the tree is n, fixaugmentations(propagate=True)
    # will consume n stack frames. Yet another reason to get rid of
    # propagate=True altogether and do everything interatively
    sys.setrecursionlimit(n+20)

    elements = range(n)
    print("Benchmarking %s with %i elements:" % (name, n))
    random.shuffle(elements)
    insert_took = measure(lambda: insert_elements(bst, elements))
    print("  Inserting in random order took:          %fs" % insert_took.total_seconds())

    random.shuffle(elements)
    search_took = measure(lambda: find_elements(bst, elements))
    print("  Searching in random order took:          %fs" % search_took.total_seconds())

    random.shuffle(elements)
    delete_took = measure(lambda: delete_elements(bst, elements))
    print("  Deleting in random order took:           %fs" % delete_took.total_seconds())

    elements = range(n)
    insert_took = measure(lambda: insert_elements(bst, elements))
    print("  Inserting in linear order took:          %fs" % insert_took.total_seconds())

    search_took = measure(lambda: find_elements(bst, elements))
    print("  Searching in linear order took:          %fs" % search_took.total_seconds())

    delete_took = measure(lambda: delete_elements(bst, elements[::-1]))
    print("  Deleting in reversed linear order took:  %fs" % delete_took.total_seconds())


if __name__ == "__main__":
    benchmark_tree('Naive BST', bstmod.BST())
    benchmark_tree('AVL BST', bstmod.AvlBst())
    benchmark_tree('AVL BST', bstmod.AvlBst(), n=10000)
    benchmark_tree('AVL BST', bstmod.AvlBst(), n=100000)