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
    A = li()

    ans = -INF
    for t in range(N):
        a = 0
        ascore = -INF
        for i in range(N):
            if i == t:
                continue
            ot, to = min(t, i), max(t, i)
            sum_aok = sum(A[ot+1:to+1:2])
            if ascore < sum_aok:
                ascore = sum_aok
                a = i

        ot, to = min(t, a), max(t, a)
        ans = max(ans, sum(A[ot:to+1:2]))
    print(ans)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
