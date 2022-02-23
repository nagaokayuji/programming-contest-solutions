import sys
from pprint import pprint
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from functools import lru_cache
from math import sqrt, gcd, factorial, pi, cos, sin, hypot
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
# from numba import njit, void, b1, i1, i4, i8, f8
# import numpy as np
# from numpy import searchsorted
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def li(): return list(map(int, input().split()))
def li1(): return list(map(lambda x: int(x)-1, input().split()))
def yn(a): print("Yes" if a else "No")


def わーい():
    N, = li()
    S = [list(input()) for _ in range(2)]
    ans = 1
    i = 0
    typ = 0
    while i < N:
        if S[0][i] != S[1][i]:
            if i == 0:
                ans *= 6
            else:
                if typ == 1:
                    ans *= 2
                else:
                    ans *= 3
            typ = 0
            i += 2
        else:
            if i == 0:
                ans *= 3
            elif typ == 1:
                ans *= 2
            typ = 1
            i += 1
        ans %= MOD
    print(ans)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    わーい()
