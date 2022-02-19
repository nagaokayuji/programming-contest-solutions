import sys
from pprint import pprint
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from functools import lru_cache
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
def yn(a): print("Yes" if a else "No")


MOD = 998244353


class ModComb:
    def __init__(self, N=10**6):
        self.MOD = MOD
        self.N = N
        self.factorial = [1] * (N+1)

        for i in range(2, N+1):
            self.factorial[i] = self.factorial[i-1]*i % MOD

        self.inv_factorial = [1] * (N+1)
        # N! の逆元を先に計算
        self.inv_factorial[N] = pow(self.factorial[N], MOD-2, MOD)
        # (N-1)! / (N!) * N = 1
        for i in range(N, 1, -1):
            self.inv_factorial[i-1] = self.inv_factorial[i]*i % MOD

    def comb(self, n, k):
        if n < k or n < 0 or k < 0:
            return 0
        return (self.factorial[n] * self.inv_factorial[k] % self.MOD) * self.inv_factorial[n-k] % self.MOD

    def comb_multi(self, n, k):
        return self.comb(n+k-1, k)


def _solve():
    N, M, K = li()
    md = ModComb()
    if N == 1:
        print(M)
        return
    if M == 1:
        if N-1 == K:
            print(M)
        else:
            print(0)
        return

    ans = 0
    for k in range(K+1):
        ans += (md.comb(N-1, k) * M % MOD) * pow(M-1, N-k-1, MOD) % MOD
        ans %= MOD
    print(ans)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    _solve()
