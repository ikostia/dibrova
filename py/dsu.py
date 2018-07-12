from __future__ import print_function
from __future__ import absolute_import

import random

class BaseDSU(object):
    def __init__(self, n):
        self.parent = [i for i in xrange(n)]

    def join(self, i, j):
        il = self.find_leader(i)
        jl = self.find_leader(j)
        if random.randint(0, 1) == 1:
            self.parent[il] = jl
        else:
            self.parent[jl] = il

    def find_leader(self, i):
        while self.parent[i] != i:
            i = self.parent[i]
        return i

    def is_same_set(self, i, j):
        return self.find_leader(i) == self.find_leader(j)

class DSU(BaseDSU):
    def find_leader(self, i):
        path = []
        while self.parent[i] != i:
            path.append(i)
            i = self.parent[i]
        for el in path:
            self.parent[el] = i
        return i
