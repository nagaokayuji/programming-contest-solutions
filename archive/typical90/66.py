import sys
from numba import njit, void, b1, i1, i4, i8, f8
import numpy as np
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())
def ti(): return tuple(mi())


@njit(f8(i8, i8, i8, i8), cache=True)
def calc(l, r, nl, nr):
    cnt = 0
    all = 0
    for v in range(l, r+1):
        for nx in range(nl, nr+1):
            if v > nx:
                cnt += 1
            all += 1
    return cnt/all


@njit(f8(i8, i8[:], i8[:]), cache=True)
def solve(N, L, R):
    ans = 0
    for i in range(N):
        for j in range(i+1, N):
            ans += calc(L[i], R[i], L[j], R[j])
    return ans


N = int(input())
LR = np.array([ti() for _ in range(N)])

print(solve(N, LR[:, 0], LR[:, 1]))
