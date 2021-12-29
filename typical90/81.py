from pprint import pprint
import sys
from numba import njit, void, b1, i1, i4, i8, f8
import numpy as np
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())
def ti(): return tuple(mi())
def i(): return int(input())


N, K = mi()
AB = [ti() for _ in range(N)]


MX = 5002

field = np.zeros((MX+2, MX+2), dtype=np.int64)
for a, b in AB:
    field[a][b] += 1


@njit(void(i8[:, :]))
def nxf(field):
    for i in range(MX):
        for j in range(MX):
            field[i][j+1] += field[i][j]
    for i in range(MX):
        for j in range(MX):
            field[i+1][j] += field[i][j]

    ans = 0
    for i in range(MX-K):
        for j in range(MX-K):
            ans = max(ans, field[i][j]+field[i+K+1][j+K+1] -
                      field[i][j+K+1]-field[i+K+1][j])

    print(ans)


nxf(field)
