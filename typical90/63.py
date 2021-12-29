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


H, W = mi()
P = [li() for _ in range(H)]

'''
2^8  *  ? 
'''
ans = 0
for row_bits in range(1, 1 << H):
    row_inds = []
    for row in range(H):
        if row_bits >> row & 1:
            row_inds.append(row)

    # 縦の同じやつリスト
    row_sames = []
    for i in range(W):
        val = P[row_inds[0]][i]
        ok = True
        for j in row_inds:
            if P[j][i] != val:
                ok = False
                break

        if ok:
            row_sames.append(val)
    if not row_sames:
        continue

    col_sames_max = max(Counter(row_sames).values())
    ans = max(ans, len(row_inds) * col_sames_max)

print(ans)
