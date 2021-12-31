from pprint import pprint
import sys
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial, pi, cos, sin
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
from numba import njit, void, b1, i1, i4, i8, f8
import numpy as np
# from numpy import searchsorted
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


@njit(i8(i8), cache=True)
def solve(N):
    table = np.zeros(N+1, dtype=np.int64)
    for i in range(1, N+1):
        table[i] = i
    i = 2
    while i*i <= N:
        if table[i] == i:
            j = i
            while j <= N:
                if table[j] == j:
                    table[j] = i
                j += i
        i += 1
    ans = 0
    for k in range(1, N+1):
        _n = k
        cnt = 1

        while _n > 1:
            pc = 0
            dvs = table[_n]
            while _n > 1 and _n % dvs == 0:
                pc += 1
                _n //= dvs
            cnt *= pc+1

        ans += cnt*k

    return ans


def _solve():
    N = readi()
    print(solve(N))
    return


if __name__ == '__main__':
    _solve()
