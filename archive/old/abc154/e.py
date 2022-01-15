
from numba import njit
import numpy as np

N = np.array(list(map(int, list(input()))))
K = int(input())


# @njit("i8(i8[:],i8)")
def solve(N, K):
    dp = np.zeros((2, K+2), dtype=np.int64)
    dp[0][0] = 1
    for now in N:
        newdp = np.zeros((2, K+2), dtype=np.int64)
        for less in range(2):
            for k in range(K+1):
                for i in range(10):
                    if now < i and less == 0:
                        continue
                    less2 = 1 if less == 1 or i < now else 0
                    k2 = k if i == 0 else k+1
                    newdp[less2][k2] += dp[less][k]
        dp = newdp
    return dp[0][K] + dp[1][K]


print(solve(N, K))
