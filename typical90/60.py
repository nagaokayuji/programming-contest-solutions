from pprint import pprint
import sys
from bisect import bisect_left, bisect_right
# from numba import njit, void, b1, i1, i4, i8, f8
# import numpy as np
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())
def ti(): return tuple(mi())


N = int(input())
A = li()


def lis(A):
    N = len(A)
    dp = [INF]*(N+1)
    dp[0] = -1
    ret = []
    for a in A:
        ind = bisect_left(dp, a)
        dp[ind] = min(a, dp[ind])
        ret.append(ind)
    return ret


dp1 = lis(A)
dp2 = lis(A[::-1])


ans = 0
for i in range(N):
    ans = max(ans, dp1[i]+dp2[N-i-1]-1)
print(ans)
