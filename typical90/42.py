from pprint import pprint
import sys
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
# from numba import njit, b1, i1, i4, i8, f8
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())
def ti(): return tuple(mi())


MOD = 10**9 + 7


# keta
K = int(input())


# dp[ketawa][mod9]
dp = defaultdict(int)
dp = [[0]*9 for _ in range(K+1)]

dp[0][0] = 1
for ketawa in range(K):
    for mod9 in range(9):
        for ad in range(1, 10):
            if ketawa+ad > K:
                break
            dp[ketawa+ad][(mod9+ad) % 9] += dp[ketawa][mod9]
            dp[ketawa+ad][(mod9+ad) % 9] %= MOD

pprint(dp[K][0])
