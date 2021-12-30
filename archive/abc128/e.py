from pprint import pprint
import sys
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
# from numba import njit, void, b1, i1, i4, i8, f8
# import numpy as np
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def solve():
    N, Q = readmi()
    STX = [readti() for _ in range(N)]
    D = [readi() for _ in range(Q)]
    xs = []
    for s, t, x in STX:
        xs.append((s-x, t-x-1, x))
    xs.sort(key=lambda x: (x[1], x[0]))
    # print(xs)
    anss = []
    l = 0
    for d in D:
        while l < N and d > xs[l][1]:
            l += 1

        # if l < N:
        #     print(d, xs[l][0], xs[l][1])
        # else:
        #     print("no")
        if l < N and xs[l][0] <= d <= xs[l][1]:
            anss.append(xs[l][2])
        else:
            anss.append(-1)
    print(*anss)

    return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    solve()
