L = input()
N = len(L)
MOD = 10**9+7

dp = [[0]*2 for _ in range(N+1)]
dp[0][1] = 1

for i in range(N):
    dp[i+1][0] += dp[i][0]*3 % MOD
    if L[i] == '1':
        dp[i+1][0] += dp[i][1] % MOD
        dp[i+1][1] += dp[i][1]*2 % MOD
    else:
        dp[i+1][1] += dp[i][1] % MOD

    dp[i][0] %= MOD
    dp[i][1] %= MOD
    dp[i+1][0] %= MOD
    dp[i+1][1] %= MOD

ans = (dp[N][0]+dp[N][1]) % MOD
print(ans)
