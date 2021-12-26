from pprint import pprint
import sys
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


N = int(input())
A = li()

INF = 10**9
dp = [[INF]*(2*N) for _ in range(2*N)]

for i in range(2*N-1):
    dp[i][i+1] = abs(A[i]-A[i+1])

for width in range(2, 2*N+1, 2):
    for l in range(2*N-1):
        r = l+width-1
        if r >= 2*N:
            break
        dp[l][r] = min(dp[l][r], dp[l+1][r-1] + abs(A[l]-A[r]))
        for mid in range(l+1, r, 2):
            dp[l][r] = min(dp[l][r], dp[l][mid]+dp[mid+1][r])

print(dp[0][2*N-1])
