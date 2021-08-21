from collections import defaultdict
N = int(input())
S = list(input())


MOD = 998244353


# dp = [[0] * (1 << 10) for _ in range(N+1)]
dp = [[([0 for _ in range(10)]) for _ in range(1 << 10)] for _ in range(N+2)]
for i in range(0, 10):
    dp[0][0][i] = 0


def od(x): return ord(x) - ord('A')


A = list(map(od, S))


# 10^7
for i, x in enumerate(A):
    dp[i+1][1 << x][x] += 1
    dp[i+1][1 << x][x] %= MOD

    for subset in range(1 << 10):
        if (subset >> x) & 1 == 0:  # x を追加する場合
            for j in range(10):
                if x == j:
                    continue
                dp[i+1][subset | (1 << x)][x] += dp[i][subset][j] % MOD
        # すでにある場合
        if (subset >> x) & 1 == 1:
            dp[i+1][subset | (1 << x)][x] += dp[i][subset][x] % MOD

        for j in range(10):
            dp[i+1][subset][j] += dp[i][subset][j] % MOD  # 選ばない場合


ans = 0
for i in range(10):
    for j in range(1 << 10):
        ans += dp[N][j][i] % MOD
print(ans % MOD)
