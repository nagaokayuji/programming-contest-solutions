import numba
import numpy as np
N = int(input())
S = list(input())
A = list(map(lambda x: ord(x) - ord('A'), S))
MOD = 998244353


@numba.njit(cache=True)
def solve(A):
    dp = np.zeros((N+1, 1 << 10, 10), dtype=np.int64)
    for i, x in enumerate(A):
        dp[i] %= MOD
        dp[i+1][1 << x][x] += 1
        for subset in range(1 << 10):
            if (subset >> x) & 1 == 0:  # x を追加する場合
                for j in range(10):
                    dp[i+1][subset | (1 << x)][x] += dp[i][subset][j]
            else:  # すでにある場合
                dp[i+1][subset | (1 << x)][x] += dp[i][subset][x]

            for j in range(10):
                dp[i+1][subset][j] += dp[i][subset][j]   # 選ばない場合

    return dp[N].sum() % MOD


print(solve(np.array(A)))
