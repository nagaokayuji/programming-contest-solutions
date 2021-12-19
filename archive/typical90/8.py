from pprint import pprint
N = int(input())
S = list(input())
MOD = 10**9 + 7

TARGET = "_atcoder"
M = len(TARGET)

dp = [[0] * (M+1) for _ in range(N+1)]
dp[0][0] = 1

for i in range(N):
    for j in range(M):
        dp[i+1][j] = max(dp[i+1][j], dp[i][j])
        if S[i] == TARGET[j]:
            dp[i+1][j] += dp[i][j-1]
            dp[i+1][j] %= MOD

print(dp[N][M-1] % MOD)
