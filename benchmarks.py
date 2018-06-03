from __future__ import print_function
from __future__ import absolute_import

import sys
import datetime
import random

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


def benchmark_tree(name, bst, elems_to_insert, elems_to_delete):
    n = len(elems_to_insert)
    print("Benchmarking %s with %i elements:" % (name, n))
    insert_took = measure(lambda: insert_elements(bst, elems_to_insert))
    print("  Inserting took:          %fs" % insert_took.total_seconds())

    search_took = measure(lambda: find_elements(bst, elems_to_insert))
    print("  Searching took:          %fs" % search_took.total_seconds())

    delete_took = measure(lambda: delete_elements(bst, elems_to_delete))
    print("  Deleting took:           %fs" % delete_took.total_seconds())

if __name__ == "__main__":
    ei100, ed100 = range(100), range(100)
    ei1000, ed1000 = range(1000), range(1000)
    ei10000, ed10000 = range(10000), range(10000)
    ei100000, ed100000 = range(100000), range(100000)
    eil1000, edl1000 = range(1000), range(999, 0, -1)

    for lst in [ei100, ed100, ei1000, ed1000, ei10000, ed10000, ei100000, ed100000]:
        random.shuffle(lst)

    benchmark_tree('Naive BST sequential order', bstmod.BST(), eil1000, edl1000)
    benchmark_tree('AVL BST sequential order', bstmod.AvlBst(), eil1000, edl1000)
    benchmark_tree('Naive BST random order', bstmod.BST(), ei1000, ed1000)
    benchmark_tree('AVL BST random order', bstmod.AvlBst(), ei100, ed100)
    benchmark_tree('AVL BST random order', bstmod.AvlBst(), ei1000, ed1000)
    benchmark_tree('AVL BST random order', bstmod.AvlBst(), ei10000, ed10000)
    benchmark_tree('AVL BST random order', bstmod.AvlBst(), ei100000, ed100000)