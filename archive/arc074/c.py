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
def readmi1(): return map(lambda x: int(x)-1, input().split())
def readli(): return list(readmi())
def readli1(): return list(readmi1())
def readti(): return tuple(readmi())
def readti1(): return tuple(readmi1())
def readi(): return int(input())


def _solve():
    H, W = readti()
    a, b = min(H, W), max(H, W)
    if a % 3 == 0 or b % 3 == 0:
        print(0)
        return

    ans = INF
    S3 = H*W/3

    for _ in range(2):
        s1 = b * ((a+2)//3)
        s2 = ((b+1)//2) * (a - ((a+2)//3))
        s3 = a*b-s1-s2
        ans = min(ans, (max(s1, s2, s3)-min(s1, s2, s3)))

        s1 = b * ((a+2)//3)
        s2 = b * (a - ((a+2)//3)+1)//2
        s3 = a*b-s1-s2
        ans = min(ans, (max(s1, s2, s3)-min(s1, s2, s3)))

        if a//3 > 0:
            s1 = b * (a//3)
            s2 = ((b+1)//2) * (a - (a//3))
            s3 = a*b-s1-s2
            ans = min(ans, (max(s1, s2, s3)-min(s1, s2, s3)))
        if b//2 > 0:
            s1 = b * (a//3)
            s2 = (b//2) * (a - (a//3))
            s3 = a*b-s1-s2
            ans = min(ans, (max(s1, s2, s3)-min(s1, s2, s3)))
        a, b = b, a
    print(ans)
    return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
