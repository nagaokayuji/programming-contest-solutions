from pprint import pprint
import sys
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial
# from math import isqrt, prod,comb  # python3.8用(notpypy)
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
#from numba import njit, b1, i1, i4, i8, f8
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


H, W = mi()
Q = int(input())


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


dsu = DSU((W+1)*(H+1)+1)

field = [[False]*(W+1) for _ in range(H+1)]


def rc(r, c):
    return r*W+c


dx = [-1, 0, 0, 1]
dy = [0, -1, 1, 0]

for _ in range(Q):
    q = tuple(mi())
    if q[0] == 1:
        r, c = q[1], q[2]
        field[r][c] = True
        for dir in range(4):
            nr, nc = r + dx[dir], c + dy[dir]
            if 1 <= nr <= H and 1 <= nc <= W:
                if field[nr][nc]:
                    dsu.merge(rc(r, c), rc(nr, nc))

    else:
        ra, ca, rb, cb = q[1], q[2], q[3], q[4]
        t = dsu.same(rc(ra, ca), rc(rb, cb)
                     ) and field[ra][ca] and field[rb][cb]
        if t:
            print("Yes")
        else:
            print("No")
