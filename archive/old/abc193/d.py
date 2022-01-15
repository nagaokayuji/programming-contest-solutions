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


def score(c):
    ret = 0
    for i in range(1, 10):
        ret += i * (10**(c[i]))
    return ret


def solve():
    K = int(input())
    S = list(map(int, input()[:-1]))
    T = list(map(int, input()[:-1]))
    tc = [0]*10
    ac = [0]*10
    for s in S:
        tc[s] += 1
    for t in T:
        ac[t] += 1

    chi = 0
    left_cnts = 9*K - 8
    left_cnts_comb = left_cnts * (left_cnts-1) // 2
    for tak in range(1, 10):
        if tc[tak]+ac[tak] >= K:
            continue
        tc[tak] += 1
        tak_score = score(tc)
        for aok in range(1, 10):
            if tc[aok] + ac[aok] >= K:
                continue
            ac[aok] += 1
            aok_score = score(ac)
            if tak_score > aok_score:
                # 通り数
                if aok == tak:
                    tmp = K-tc[tak]-ac[aok]+2
                    chi += tmp*(tmp-1)//2
                else:
                    chi += (K-tc[tak]+1-ac[tak]) * (K-ac[aok]+1-tc[aok])/2

            ac[aok] -= 1
        tc[tak] -= 1
    assert sum(tc) == 4
    assert sum(ac) == 4
    print(chi/left_cnts_comb)

    return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    solve()
