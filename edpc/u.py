import sys
from pprint import pprint
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial, pi, cos, sin, hypot
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
from numba import njit, void, b1, i1, i4, i8, f8
import numpy as np
# from numpy import searchsorted
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


def _solve():
    N = int(input())
    A = np.array([list(mi()) for _ in range(N)], dtype=np.int64)

    @njit(i8[:](), cache=True)
    def gr():
        grouping_points = np.zeros(1 << N, dtype=np.int64)
        for bits in range(1 << N):
            points = 0
            for i in range(N):
                if bits >> i & 1 == 0:
                    continue
                for j in range(i+1, N):
                    if bits >> j & 1 == 0:
                        continue
                    points += A[i][j]
            grouping_points[bits] = points
        return grouping_points
    grouping_points = gr()

    @njit(i8(i8[:]), cache=True)
    def solve(grouping_points):
        dp = [0]*(1 << N)  # dp[S]: usagi set S, score
        np.zeros((1 << N), dtype=np.int64)
        for s in range(1 << N):
            k = s
            while k:
                dp[s] = max(dp[s], dp[s-k] + grouping_points[k])
                k = (k-1) & s
        return dp[(1 << N)-1]

    print(solve(np.array(grouping_points, dtype=np.int64)))


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
