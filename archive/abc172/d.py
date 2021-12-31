from pprint import pprint
import sys
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial, pi, cos, sin
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
# from numba import njit, void, b1, i1, i4, i8, f8
# import numpy as np
# from numpy import searchsorted
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


class SmallestPrimeFactors:
    '''
    高速素因数分解
    最小の素数を返す
    O(N log N)
    '''

    def __init__(self, N: int):
        table = [i for i in range(N+1)]
        i = 2
        while i*i <= N:
            if table[i] == i:
                j = i
                while j <= N:
                    if table[j] == j:
                        table[j] = i
                    j += i
            i += 1
        self._table = table

    def prime_factorize(self, N: int):
        factors = []
        n = N
        while n > 1:
            factor = self._table[n]
            n //= factor
            factors.append(factor)
        return factors


def solve(N, table):
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
    spf = SmallestPrimeFactors(N)
    # t = np.array(spf._table)
    t = spf._table
    # print(t)
    print(solve(N, t))
    return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
