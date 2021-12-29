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
def i(): return int(input())


H, W = mi()
A = [li() for _ in range(H)]
B = [li() for _ in range(H)]

ans = 0
for i in range(H-1):
    for j in range(W-1):
        dif = B[i][j]-A[i][j]
        if dif:
            ans += abs(dif)
        A[i][j] += dif
        A[i][j+1] += dif
        A[i+1][j] += dif
        A[i+1][j+1] += dif

ok = True
for i in range(H):
    for j in range(W):
        ok &= A[i][j] == B[i][j]

if ok:
    print("Yes")
    print(ans)
else:
    print("No")
