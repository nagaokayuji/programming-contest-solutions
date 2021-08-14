N = int(input())
S = list(map(int, input().split()))
T = list(map(int, input().split()))

dp = [10**10] * N

for i in range(2*N):
    dp[(i+1) % N] = min(dp[i % N] + S[(i) % N], T[(i+1) % N])
print(*dp)
