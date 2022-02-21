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


def わーい():
    A, B = li()

    def f(x):
        ret = 0
        x += 1

        for i in range(41):
            t = 1 << (i+1)  # loopg t
            cnt = (x//t) * (t >> 1)
            cnt += max(0, (x % t) - (t//2))
            if cnt & 1:
                ret ^= 1 << i

        return ret

    print(f(B) ^ f(max(0, A-1)))


if __name__ == '__main__':
    わーい()
