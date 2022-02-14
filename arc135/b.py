import sys
from pprint import pprint
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from functools import lru_cache
# from numba import njit, void, b1, i1, i4, i8, f8
# import numpy as np
# from numpy import searchsorted
def input(): return sys.stdin.readline().rstrip()
def li(): return list(map(int, input().split()))
def li1(): return list(map(lambda x: int(x)-1, input().split()))
def yn(a): print("Yes" if a else "No")


def _solve():
    N, = li()
    S = li()
    if N == 1:
        yn(True)
        print(0, 0, S[0])
        return
    if N == 2:
        yn(True)
        print(S[0], 0, 0, S[1])
        return

    A = [0]*(N+2)
    for i in range(N):
        A[i+2] = S[i] - A[i] - A[i+1]

    print(*A)


if __name__ == '__main__':
    _solve()
