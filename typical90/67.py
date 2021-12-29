from pprint import pprint
import sys
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
# from numba import njit, void, b1, i1, i4, i8, f8
# import numpy as np
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())
def ti(): return tuple(mi())


N, K = mi()
N = list(map(int, list(str(N))))


def op(n):
    # parse val
    val = 0
    b = 1
    while n:
        val += b * n.pop()
        b *= 8

    # to 9
    ret = []
    while val:
        x = val % 9
        ret.append(x if x != 8 else 5)
        val //= 9
    while ret and ret[-1] == 0:
        ret.pop()
    return ret[::-1]


ans = N
for _ in range(K):
    ans = op(ans)
    if not ans:
        print(0)
        exit()

print("".join(map(str, ans)))
