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


ans = 0


def _solve():
    N = int(input())
    A = [list(mi()) for _ in range(2*N-1)]
    if N == 1:
        print(A[0][0])
        return

    def dfs(nowi=0, matched=0, xorsum=0):
        global ans
        if matched == (1 << (2*N))-1:
            ans = max(ans, xorsum)
            return

        for to in range(2*N-nowi-1):
            to_ind = nowi+to+1
            if (matched >> to_ind & 1) or (matched >> nowi & 1):
                continue
            nxt = matched | (1 << to_ind) | (1 << nowi)
            nxt_i = nowi+1
            for nxi in range(nowi+1, 2*N):
                if nxt >> nxi & 1 == 0:
                    nxt_i = nxi
                    break

            dfs(nxt_i, nxt, xorsum ^ A[nowi][to])
    dfs()
    print(ans)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
