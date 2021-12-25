from pprint import pprint
import sys
from collections import defaultdict, Counter, deque
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


MOD = 10**9 + 7
N, B, K = mi()
c = li()

# dp[i,j] := 上からi桁目、mod B = j の数
dp = defaultdict(int)


dp[0, 0] = 1
for i in range(N):
    for j in range(B):
        for k in range(K):
            next = (10*j+c[k]) % B
            dp[i+1, next] = (dp[i+1, next] + dp[i, j]) % MOD

# pprint(dp)

print(dp[N, 0])
