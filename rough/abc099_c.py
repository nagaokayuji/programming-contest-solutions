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
def li(): return list(map(int, input().split()))
def li1(): return list(map(lambda x: int(x)-1, input().split()))


def _solve():
    N, = li()
    dp = [INF for _ in range(N+1)]  # 1 yen
    dp[0] = 0

    tsuka = [1]
    for six in range(N+1):
        t = 6**six
        if t > N:
            break
        tsuka.append(t)

    for nine in range(N+1):
        t = 9**nine
        if t > N:
            break
        tsuka.append(t)
    tsuka = sorted(list(set(tsuka)))

    q = deque([0])
    while(q):
        now = q.popleft()
        cnt = dp[now]
        for ad in tsuka:
            nxt = now+ad
            if nxt > N:
                continue
            if dp[nxt] > cnt+1:
                dp[nxt] = min(dp[nxt], cnt + 1)
                q.append(nxt)

    print(dp[N])


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
