import sys
from pprint import pprint
# from numba import njit, void, b1, i1, i4, i8, f8
# import numpy as np
# from numpy import searchsorted
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


def _solve():
    N = int(input())
    A = list(mi())
    counts = [0]*4
    for x in A:
        counts[x] += 1
    dp = [[[0]*(N+1) for _ in range(N+1)] for _ in range(N+1)]
    dp[0][0][0] = 0

    for three in range(N+1):
        for two in range(N+1):
            for one in range(N+1):
                if one+two+three > N:
                    continue
                '''
                dp[one+1][two-1][three] += dp[one][two][three] + two/N
                dp[one][two][three] += dp[one+1][two][three] + one/N
                '''
                if one > 0:
                    dp[one][two][three] += dp[one-1][two][three] + (one-1)/N
                    # dp[one-1][two][three] += dp[one][two][three] + one/N
                if two > 0:
                    # dp[one+1][two-1][three] += dp[one][two][three] + two/N
                    dp[one][two][three] += dp[one][two-1][three] + two/N
                if three > 0:
                    dp[one][two+1][three-1] += dp[one][two][three] + three/N

    pprint(dp)
    print(dp[counts[1]][counts[2]][counts[3]])

    return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
