from numba import njit
import numpy as np
S = list(input())

S.reverse()
S = np.array(list(map(lambda x: -1 if x == '?' else int(x), S)))


@njit("i8(i8[:])")
def solve(S):
    MOD = 10**9 + 7
    b = 1
    dp = np.zeros(13)
    dp[0] = 1
    for i, num in enumerate(S):
        newdp = np.zeros(13)
        for rem in range(13):
            if num != -1:
                newdp[(rem+num*b) % 13] += dp[rem]
            else:
                for j in range(10):
                    newdp[(rem+j*b) % 13] += dp[rem]
        dp = newdp[:] % MOD
        b = (b*10) % 13
    return dp[5]


print(solve(S))
