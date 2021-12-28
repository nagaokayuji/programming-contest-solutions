from pprint import pprint
import sys
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
# from numba import njit, void, b1, i1, i4, i8, f8
# import numpy as np
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())
def ti(): return tuple(mi())


N, S = mi()
AB = [ti() for _ in range(N)]

dp = [[False]*(S+1) for _ in range(N+1)]
dp[0][0] = True
for i, (a, b) in enumerate(AB):
    for s in range(S+1):
        if s+b <= S:
            dp[i+1][s+b] |= dp[i][s]
        if s+a <= S:
            dp[i+1][s+a] |= dp[i][s]

if not dp[N][S]:
    print("Impossible")
    exit()

back = []
now = S
for i in range(N, 0, -1):
    a, b = AB[i-1]
    if now >= a and dp[i-1][now-a]:
        back.append("A")
        now -= a
    else:
        back.append("B")
        now -= b

print("".join(back[::-1]))
