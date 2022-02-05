S = input()
T = input()
N = len(S)
M = len(T)

dp = [[0]*(M+1) for _ in range(N+1)]
for i in range(N):
    for j in range(M):
        if S[i] == T[j]:
            dp[i+1][j+1] = max(dp[i][j]+1, dp[i][j+1], dp[i+1][j])
        else:
            dp[i+1][j+1] = max(dp[i+1][j], dp[i][j+1])


ans = ""
i = N
j = M
while i+j > 0:
    now = dp[i][j]
    while i > 0 and now == dp[i-1][j]:
        i -= 1
    while j > 0 and now == dp[i][j-1]:
        j -= 1
    if now > dp[i-1][j-1]:
        i -= 1
        j -= 1
        ans += S[i]
        continue

print(ans[::-1])
