from pprint import pprint
import sys
from collections import defaultdict, Counter, deque
from numba import njit, b1, i1, i4, i8, f8
import numpy as np
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


N = int(input())
W = li()
B = li()


@njit(cache=True)
def gen():
    grundy = np.zeros((55, 1455), np.int64)
    for w in range(51):
        for b in range(1442):
            mx = np.zeros(1455, np.int8)
            if w >= 1:
                mx[grundy[w-1][b+w]] = 1
            if b >= 2:
                for i in range(1, b//2 + 1):
                    mx[grundy[w][b-i]] = 1

            for i in range(1455):
                if mx[i] == 0:
                    grundy[w][b] = i
                    break
    return grundy


grundy = gen()

ans = 0
for w, b in zip(W, B):
    ans ^= grundy[w, b]

if ans > 0:
    print('First')
else:
    print("Second")
