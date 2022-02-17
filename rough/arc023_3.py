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


def _solve():
    def nCk(n, k):
        ret = 1
        if n - k < k:
            k = n - k
        for i in range(n, n - k, -1):
            ret = ret * i % MOD
        fact = [1] * (k + 1)
        for i in range(1, k + 1):
            fact[i] = fact[i - 1] * i % MOD
        return ret % MOD * pow(fact[k], MOD - 2, MOD) % MOD
    N, = li()
    A = li()
    prv = A[0]
    i = 0
    cnt = 0
    ans = 1
    while i < N:
        cnt += 1
        st = i
        while i < N and A[i] == -1:
            i += 1
        ed = i
        if ed == N:
            break

        _from = A[st-1]
        to = A[ed]

        i = ed+1
        if st == ed:
            continue
        length = ed-st
        v = to-_from+1
        if v == 1:
            continue
        # print(v, length)
        ans = ans * (nCk(v+length-1, length)) % MOD
    print(ans)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
