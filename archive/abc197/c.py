from pprint import pprint
import sys
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
from numba import njit, void, b1, i1, i4, i8, f8
import numpy as np
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


@njit(i8(i8, i8[:]), cache=True)
def calc(N, A):
    ans = INF
    for bits in range(1 << (N-1)):
        ret = 0
        tmp = 0
        for i in range(N+1):
            tmp |= A[i]
            if bits >> i & 1 or i == N:
                ret ^= tmp
                tmp = 0
        ans = min(ans, ret)

    return ans


def solve():
    N = readi()
    A = np.array(readli()+[0])
    print(calc(N, A))
    return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    solve()
