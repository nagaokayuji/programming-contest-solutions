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


dx = [-1, 0, 0, 1]
dy = [0, -1, 1, 0]


def _solve():
    # N x M field
    N, M, R, C = mi()
    R -= 1
    C -= 1
    # fill black one row or col
    field = [input() for _ in range(N)]
    if field[R][C] == "B":
        print(0)
        return

    blacks = deque()
    for i in range(N):
        for j in range(M):
            if field[i][j] == "B":
                blacks.append((i, j, 0))
    dp = [[INF]*M for _ in range(N)]
    visited = [[False]*M for _ in range(N)]
    while blacks:
        r, c, count = blacks.popleft()
        visited[r][c] = True
        dp[r][c] = min(dp[r][c], count)
        for tor in range(N):
            if not visited[tor][c] and dp[tor][c] > count+1:
                visited[tor][c] = True
                blacks.append((tor, c, count+1))
            dp[tor][c] = min(dp[tor][c], count+1)
        for toc in range(M):
            if not visited[r][toc] and dp[r][toc] > count+1:
                visited[r][toc] = True
                blacks.append((r, toc, count+1))
            dp[r][toc] = min(dp[r][toc], count+1)

    ans = dp[R][C]
    if ans == INF:
        print(-1)
    else:
        print(ans)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    Q = int(input())
    for _ in range(Q):
        _solve()
