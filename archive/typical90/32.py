import numpy as np
from pprint import pprint
import sys
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from numba import jit, njit, b1, i1, i4, i8, f8
INF = 10**16
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


N = int(input())
A = np.array([li() for _ in range(N)])
M = int(input())
XY = np.array([li() for _ in range(M)])
ok = np.array([[True] * N for _ in range(N)])

if N == 1:
    print(A[0][0])
    exit()

for x, y in XY:
    ok[x-1][y-1] = False
    ok[y-1][x-1] = False
perm = np.array(list(permutations(range(N))), dtype=np.int64)


@jit("i8(i8, i8[:,:], b1[:,:], i8[:,:])")
def solve(N, A, ok, perm):
    ans = INF
    for permi in range(len(perm)):
        order = perm[permi]
        time = 0
        bad = False
        for i in range(N-1):
            if not ok[order[i]][order[i+1]]:
                bad = True
                break
        if bad:
            continue
        for i in range(N):
            time += A[order[i]][i]
        ans = min(ans, time)

    if ans == INF:
        return -1
    return ans


print(solve(N, A, ok, perm))
