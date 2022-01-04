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
def inputmi(): return map(int, input().split())
def inputmi1(): return map(lambda x: int(x)-1, input().split())
def inputli(): return list(inputmi())
def inputli1(): return list(inputmi1())
def inputti(): return tuple(inputmi())
def inputti1(): return tuple(inputmi1())
def inputi(): return int(input())


def _solve():
    N, K = inputti()
    LR = [inputti() for _ in range(K)]

    dp = [0]*(N+2)
    dp[0] = 1
    for i in range(1, N):
        dp[i] += dp[i-1]
        dp[i] %= MOD
        for l, r in LR:
            if i+l <= N:
                dp[i+l] += dp[i]
            if i+r+1 <= N:
                dp[i+r+1] -= dp[i]
    print(dp[N] % MOD)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 998244353
    _solve()
