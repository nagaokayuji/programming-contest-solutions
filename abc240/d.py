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
    N, = li()
    A = li()

    stack = []
    ret = 0
    for x in A:
        stack.append((x, 1))
        ret += 1
        cnt = 0
        while stack and stack[-1][0] == x:
            backx, c = stack.pop()
            cnt += c

        if cnt % x != 0:
            stack.append((x, cnt % x))
        ret -= x * (cnt//x)

        print(ret)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
