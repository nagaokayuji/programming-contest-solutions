S = input()
MOD = 10**9 + 7
N = len(S)
dp = [0] * (N+2)

dp[0] = 1
INF = 10**20

ans = 0

for i in range(N):
    for j in range(i, -1, -1):
        dp[i+2] += dp[j]
        dp[i+2] %= MOD

        if j and S[j-1] == S[i]:
            break

    ans += dp[i+2]
    ans %= MOD

print(ans % MOD)
