import numpy as np
from numba import njit
N, M = map(int, input().split())
A = np.array(list(map(int, input().split())))


@njit("b1(i8[:],i8)")
def isok(A, x):
    '''
    mex が x 以下
    '''

    cnts = np.zeros(N+1)
    ex = 0
    for i in range(M):
        if cnts[A[i]] == 0 and A[i] <= x:
            ex += 1
        cnts[A[i]] += 1
    if ex <= x:
        return True

    for i in range(M, N):
        if cnts[A[i]] == 0 and A[i] <= x:
            ex += 1
        cnts[A[i]] += 1
        if cnts[A[i-M]] == 1 and A[i-M] <= x:
            ex -= 1
        if ex <= x:
            return True
    return False


@njit("void(i8[:])")
def solve(A):
    ok = N
    ng = -1

    while ok-ng > 1:
        mid = (ok+ng)//2
        if isok(A, mid):
            ok = mid
        else:
            ng = mid
    print(ok)


solve(A)
