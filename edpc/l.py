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
    N = int(input())
    A = list(mi())

    dp = [[0]*(N+1) for _ in range(N+1)]
    # [l,r)
    for length in range(1, N+1):
        turn_taro = (N-length) % 2 == 0
        for left in range(N-length+1):
            right = left+length
            pl = dp[left+1][right]
            pr = dp[left][right-1]
            vl = A[left]
            vr = A[right-1]

            if turn_taro:  # plus
                dp[left][right] = max(pl+vl, pr+vr)

            else:  # minus
                dp[left][right] = min(pl-vl, pr-vr)

    print(dp[0][N])


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
