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
    a, N = mi()
    MX = 10*N+100
    dp = [INF] * (10*N+100)
    dp[1] = 0

    stack = deque()
    stack.append(1)
    while stack:
        now = stack.popleft()
        if now >= 10 and now % 10 != 0:
            s = str(now)
            nx = int(s[-1]+s[:-1])
            if nx < MX and dp[nx] > dp[now]+1:
                dp[nx] = min(dp[nx], dp[now]+1)
                stack.append(nx)
        nx = now*a
        if nx < MX and dp[nx] > dp[now]+1:
            dp[nx] = min(dp[nx], dp[now]+1)
            stack.append(nx)

    ans = dp[N]
    if ans == INF:
        print(-1)
    else:
        print(ans)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
