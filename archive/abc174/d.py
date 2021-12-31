import sys
from pprint import pprint
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial, pi, cos, sin
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
# from numba import njit, void, b1, i1, i4, i8, f8
# import numpy as np
# from numpy import searchsorted
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    N = readi()
    C = input()

    # OK:
    # rrrrwwww
    # rrrrrrrr
    # wwwwwwww
    rc = [0]
    wc = [0]
    for i, x in enumerate(C):
        if x == 'R':
            rc.append(rc[-1]+1)
            wc.append(wc[-1])
        else:
            rc.append(rc[-1])
            wc.append(wc[-1]+1)
    # print(*rc)
    # print(*wc)
    ans = min(rc[-1], wc[-1])
    for border in range(N):
        leftw = wc[border]
        rightr = rc[-1]-rc[border]
        ans = min(ans, max(leftw, rightr))
    print(ans)
    return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
