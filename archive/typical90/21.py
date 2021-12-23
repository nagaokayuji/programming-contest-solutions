from typing import List
from pprint import pprint
import sys
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
# from numba import njit, b1, i1, i4, i8, f8
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


class SCC:
    def __init__(self, n: int):
        ''' n: num of nodes'''
        self.n = n
        self.g = [[] for _ in range(n)]

    def from_graph(self, g: List[List[int]]):
        self.n = len(g)
        self.g = g.copy()

    def scc():
        pass


class RollingHash():
    def __init__(self, s: str, base=10007, mod=2**61 - 1):
        n = len(s)
        self.mod = mod
        self.hash = hash = [0]*(n+1)
        self.pw = pw = [1]*(len(s)+1)

        for i in range(n):
            pw[i+1] = pw[i] * base % mod

        for i in range(n):
            hash[i+1] = (hash[i] * base + ord(s[i])) % mod

    def get(self, l: int, r: int):
        '''
        get hash of [l,r)  (0-indexed)
        '''
        return (self.hash[r] - self.hash[l] * self.pw[r-l]) % self.mod


rh = RollingHash("hogehogestring")
assert(rh.get(0, 1) == rh.get(4, 5))
assert(rh.get(0, 2) == rh.get(4, 6))
assert(rh.get(0, 3) == rh.get(4, 7))
assert(rh.get(0, 4) == rh.get(4, 8))

# N, M = mi()
# AB = [tuple(mi()) for _ in range(M)]

# g = [[] for _ in range(N)]
# for a, b in AB:
#     g[a-1].append(b-1)

print(rh.get(0, 1))
print(rh.get(0, 2))
print(rh.get(0, 3))
print(rh.get(0, 4))
print(rh.get(0, 5))
print(rh.get(4, 5))
print(rh.get(4, 6))
print(rh.get(4, 7))
print(rh.get(4, 8))
print(rh.get(4, 9))
