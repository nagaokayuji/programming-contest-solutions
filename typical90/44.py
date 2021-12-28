from pprint import pprint
import sys
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
# from numba import njit, void, b1, i1, i4, i8, f8
import numba
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())
def ti(): return tuple(mi())


N, Q = mi()
A = li()
Qs = [ti() for _ in range(Q)]
shift = 0

for t, x, y in Qs:
    x -= 1
    y -= 1
    if t == 1:
        A[(x+shift) % N], A[(y+shift) % N] = A[(y+shift) % N], A[(x+shift) % N]

    elif t == 2:
        shift -= 1
        shift = (shift+N) % N

    else:
        assert t == 3
        print(A[(x+shift) % N])
