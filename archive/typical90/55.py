import sys
# from numba import njit, void, b1, i1, i4, i8, f8
# import numpy as np
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())
def ti(): return tuple(mi())


N, P, Q = mi()
A = li()

ans = 0
for i in range(N):
    prd = A[i]
    for j in range(i+1, N):
        prd2 = prd*A[j] % P
        for k in range(j+1, N):
            prd3 = prd2*A[k] % P
            for l in range(k+1, N):
                prd4 = prd3*A[l] % P
                for m in range(l+1, N):
                    prd5 = prd4*A[m] % P
                    if prd5 == Q:
                        ans += 1
print(ans)
