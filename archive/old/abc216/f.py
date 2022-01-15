import numpy as np
from numba import njit

N = int(input())
A = np.array(list(map(int, input().split())))
B = np.array(list(map(int, input().split())))
ab = list(zip(A, B))
ab.sort()
A = np.array(list(map(lambda x: x[0], ab)))
B = np.array(list(map(lambda x: x[1], ab)))
MOD = 998244353
MX = 5005


@njit("void(i8,i8[:],i8[:])")
def solve(N, A, B):
    dp = np.zeros(MX, np.int64)
    dp[0] = 1
    ans = 0
    for a, b in zip(A, B):
        dp2 = dp.copy()
        if a >= b:
            ans += dp[:a-b+1].sum()

        for i in range(MX - b):
            dp2[i+b] += dp[i]
        dp2 %= MOD
        dp = dp2.copy()
        ans %= MOD
    print(ans % MOD)


solve(N, A, B)
