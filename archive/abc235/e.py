import sys
from pprint import pprint
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial, pi, cos, sin, hypot
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
# from numba import njit, void, b1, i1, i4, i8, f8
# import numpy as np
# from numpy import searchsorted
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


class DSU:
    def __init__(self, n: int = 0):
        self._n = n
        self.parent_or_size = [-1] * n

    def merge(self, a: int, b: int):
        x = self.leader(a)
        y = self.leader(b)

        if x == y:
            return x

        if -self.parent_or_size[x] < -self.parent_or_size[y]:
            x, y = y, x

        self.parent_or_size[x] += self.parent_or_size[y]
        self.parent_or_size[y] = x
        return x

    def same(self, a: int, b: int):
        return self.leader(a) == self.leader(b)

    def leader(self, a: int):
        parent = self.parent_or_size[a]
        while parent >= 0:
            if self.parent_or_size[parent] < 0:
                return parent
            self.parent_or_size[a], a, parent = (
                self.parent_or_size[parent],
                self.parent_or_size[parent],
                self.parent_or_size[self.parent_or_size[parent]]
            )

        return a

    def size(self, a: int):
        return -self.parent_or_size[self.leader(a)]

    def groups(self):
        leader_buf = [self.leader(i) for i in range(self._n)]
        result = [[] for _ in range(self._n)]
        for i in range(self._n):
            result[leader_buf[i]].append(i)
        return list(filter(lambda r: r, result))


def _solve():
    N, M, Q = mi()
    abc = [tuple(mi()) for _ in range(M)]
    abc.sort(key=lambda x: x[2])
    uvw = [tuple(mi()) for _ in range(Q)]

    abcanduvw = []
    for i, (a, b, c) in enumerate(abc):
        a -= 1
        b -= 1
        abcanduvw.append((a, b, c, True, i))
    for i, (a, b, c) in enumerate(uvw):
        a -= 1
        b -= 1
        abcanduvw.append((a, b, c, False, i))

    abcanduvw.sort(key=lambda x: x[2])

    dsu = DSU(N)

    anss = [False]*Q

    for a, b, w, orig, index in abcanduvw:
        if not dsu.same(a, b):
            if orig:
                dsu.merge(a, b)
            else:
                anss[index] = True

    for ans in anss:
        if ans:
            print("Yes")
        else:
            print("No")


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
