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
    K = list(map(int, list(input())))
    N = len(K)
    D = int(input())
    dp = defaultdict(int)
    dp = [[[0] * D for _ in range(2)] for _ in range(N+1)]

    # dp[i,j,k] := i文字目, j:strict, k: modD の個数

    dp[0][1][0] = 1

    for i, v in enumerate(K):
        for strict in range(2):
            for to in range(10):
                if strict and to > v:
                    break
                nxstrict = 1 if strict and to == v else 0

                for modd in range(D):
                    nxmod = (modd + to) % D
                    dp[i+1][nxstrict][nxmod] += dp[i][strict][modd]
                    dp[i+1][nxstrict][nxmod] %= MOD

    print((dp[N][0][0]+dp[N][1][0]-1) % MOD)

    return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
