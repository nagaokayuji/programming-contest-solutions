import sys
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


N = int(input())
DCS = [tuple(mi()) for _ in range(N)]
DCS.sort()
MXD = DCS[-1][0]

dp = [0] * (MXD+1)
for d, c, s in DCS:
    dp2 = dp.copy()
    for today in range(MXD):
        if today+c > d:
            continue
        dp2[today+c] = max(dp2[today+c], dp[today]+s, dp[today+c])

    dp = dp2

print(max(dp))
