from pprint import pprint
import sys
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial
# from math import isqrt, prod,comb  # python3.8用(notpypy)
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
#from numba import njit, b1, i1, i4, i8, f8
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


N = int(input())
DCS = [tuple(mi()) for _ in range(N)]
DCS.sort()
MXD = DCS[-1][0]

dp = [0] * (MXD+1)
for d, c, s in DCS:
    dp2 = dp.copy()
    for today in range(MXD):
        if today+c > d:
            continue
        dp2[today+c] = max(dp2[today+c], dp[today]+s, dp[today+c])

    dp = dp2

print(max(dp))
