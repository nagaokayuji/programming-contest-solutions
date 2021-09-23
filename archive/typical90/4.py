import numpy as np
from numba import njit
import sys
H, W = map(int, input().split())

A = np.array([list(map(int, sys.stdin.readline().split())) for _ in range(H)])


@njit("i8[:,:](i8[:,:])")
def solve(A):
    r = np.zeros(H, dtype=np.int64)
    c = np.zeros(W, dtype=np.int64)

    for i in range(H):
        r[i] = np.sum(A[i])

    t = A.T
    for j in range(W):
        c[j] = np.sum(t[j])

    anss = np.zeros((H, W), dtype=np.int64)
    for i in range(H):
        for j in range(W):
            anss[i][j] = r[i] + c[j] - A[i][j]
    return anss


anss = solve(A)

for row in anss:
    print(' '.join(map(str, row.tolist())))
