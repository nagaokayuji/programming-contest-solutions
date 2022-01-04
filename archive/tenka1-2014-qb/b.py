import sys
# from numba import njit, void, b1, i1, i4, i8, f8
# import numpy as np
# from numpy import searchsorted
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readmi1(): return map(lambda x: int(x)-1, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    N = readi()
    S = input()
    M = len(S)
    T = [input() for _ in range(N)]
    dp = [0]*(M+1)
    dp[0] = 1

    for s in range(M):
        dp[s] %= MOD
        for t in T:
            lt = len(t)
            if S[s:s+lt] == t:
                dp[s+lt] += dp[s]
                dp[s+lt] %= MOD

    print(dp[-1])

    return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
