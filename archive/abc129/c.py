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


def _solve():
    N, M = mi()
    A = [int(input()) for _ in range(M)]
    ok = [True]*(N+10)
    for a in A:
        ok[a] = False

    dp = [0]*(N+10)
    dp[0] = 1
    for i in range(N):
        dp[i] %= MOD
        if ok[i+1]:
            dp[i+1] += dp[i]
        if ok[i+2]:
            dp[i+2] += dp[i]

    print(dp[N] % MOD)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
