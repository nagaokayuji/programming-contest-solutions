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
def yn(a): print("Yes" if a else "No")


def _solve():
    ''''''
    D, G = li()
    G //= 100
    PC = [tuple(li()) for _ in range(D)]
    ans = INF
    for bts in range(1 << D):
        s = 0
        count = 0
        for i, (p, c) in enumerate(PC):
            if bts >> i & 1:
                s += c//100
                s += (i+1)*p
                count += p
        if s >= G:
            ans = min(ans, count)
            continue

        for i in range(D-1, -1, -1):
            sc = i+1
            p, _ = PC[i]
            if (bts >> i & 1) == 0:
                left = max(0, G-s)
                if sc*p >= left:
                    count += (left+sc-1)//sc
                    s = G
                    break
                else:
                    count += p
                    s += p*sc

        if s >= G:
            ans = min(ans, count)
    print(ans)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
