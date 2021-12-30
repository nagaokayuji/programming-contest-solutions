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
    a, b = readmi()
    c, d = readmi()
    ans = -1111
    for x in range(a, b+1):
        for y in range(c, d+1):
            ans = max(ans, x-y)
    print(ans)
    return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    solve()
