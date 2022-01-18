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
    N, K = mi()
    A = list(mi())
    dp = [True]*(K+1)
    dp[0] = False  # 0で回されたら負け

    for i in range(1, K+1):
        # false になるやつで回すのが最適
        iret = False
        for x in A:
            if i-x >= 0:
                iret |= not dp[i-x]
        dp[i] = iret

    # print(*dp)
    if dp[K]:
        print("First")
    else:
        print("Second")


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
