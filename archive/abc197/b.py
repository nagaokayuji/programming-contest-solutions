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
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def solve():
    H, W, X, Y = readmi()
    S = [input() for _ in range(H)]
    rs = [[0]*W for _ in range(H)]
    cs = [[0]*W for _ in range(H)]
    for i in range(H):
        rs[i][0] = 1 if S[i][0] == '.' else 0
        for j in range(1, W):
            if S[i][j] == '.':
                rs[i][j] = rs[i][j-1]+1
            else:
                rs[i][j] = 0
        for j in range(W-2, -1, -1):
            if S[i][j] == '.':
                rs[i][j] = max(rs[i][j], rs[i][j+1])

    for j in range(W):
        cs[0][j] = 1 if S[0][j] == '.' else 0
        for i in range(1, H):
            if S[i][j] == '.':
                cs[i][j] = cs[i-1][j]+1
            else:
                cs[i][j] = 0
        for i in range(H-2, -1, -1):
            if S[i][j] == '.':
                cs[i][j] = max(cs[i][j], cs[i+1][j])

    print(rs[X-1][Y-1]+cs[X-1][Y-1]-1)

    return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    solve()
