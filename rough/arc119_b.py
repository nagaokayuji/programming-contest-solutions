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
    N = int(input())
    S = list(input())
    T = list(input())
    if S.count('1') != T.count('1'):
        print(-1)
        return

    szero = []
    tzero = []
    for i in range(N):
        if S[i] == '0':
            szero.append(i)
        if T[i] == '0':
            tzero.append(i)

    assert (len(szero) == len(tzero))
    ans = len(list(filter(lambda x: x[0] != x[1], zip(szero, tzero))))
    print(ans)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
