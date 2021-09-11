S = input()
MOD = 10**9 + 7

INF = 10**14
N = len(S)
nxc = [[N+1] * 26 for _ in range(N+3)]

for i in range(N-1, -1, -1):
    ind = ord(S[i]) - ord('a')  # 0 ~ 25
    nxc[i] = nxc[i+1][:]
    nxc[i][ind] = i

dp = [0]*(N+4)
dp[-2] = 1


for i in range(-2, N):
    for nx in nxc[i+2]:
        dp[nx] += dp[i]
        dp[nx] %= MOD

print(sum(dp[0:N+1]) % MOD)
