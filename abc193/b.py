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
    N = readi()
    APX = [readti() for _ in range(N)]
    ans = INF
    for a, p, x in APX:
        if a >= x:
            continue
        ans = min(ans, p)
    if ans == INF:
        print(-1)
        return
    print(ans)

    return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    solve()
