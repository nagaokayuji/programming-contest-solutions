import sys
from pprint import pprint
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


N, M, K = mi()

MOD = 998244353

dp = [[0]*N for _ in range(K+1)]  # dp[i][j] := i日目にjにいる通り数
dp[0][0] = 1

UV = [tuple(mi()) for _ in range(M)]
ng = [[] for _ in range(N)]
for u, v in UV:
    ng[u-1].append(v-1)
    ng[v-1].append(u-1)
for i in range(N):
    ng[i].append(i)

for d in range(K):
    s = sum(dp[d])
    for v in range(N):
        nx = s
        for ngt in ng[v]:
            nx -= dp[d][ngt]
        dp[d+1][v] += nx
        dp[d+1][v] %= MOD
# pprint(dp)
print(dp[K][0])
