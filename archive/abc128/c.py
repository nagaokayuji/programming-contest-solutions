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
    N, M = readti()
    S = [readli()[1:] for _ in range(M)]
    P = readli()
    ans = 0
    for bits in range(1 << N):
        sws = [False]*N
        for i in range(N):
            if bits >> i & 1:
                sws[i] = True

        ok = True
        for ls in range(M):
            cnt = 0
            for x in S[ls]:
                if sws[x-1]:
                    cnt += 1
            if P[ls] & 1 != cnt & 1:
                ok = False
                break
        if ok:
            ans += 1

    print(ans)

    return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    solve()
