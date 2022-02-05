import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())
def ti(): return tuple(mi())


sys.setrecursionlimit(10**7)

N = int(input())
c = input().split()
ab = [ti() for _ in range(N-1)]
g = [[] for _ in range(N)]
for a, b in ab:
    a -= 1
    b -= 1
    g[a].append(b)
    g[b].append(a)


MOD = 10**9 + 7


def dfs(now, prv):
    v1 = 1
    v2 = 1
    for nx in g[now]:
        if nx == prv:
            continue
        dfs(nx, now)
        if c[now] == 'a':
            v1 *= dp[nx][0]+dp[nx][2]
            v2 *= dp[nx][0]+dp[nx][1]+dp[nx][2]*2
        else:
            v1 *= dp[nx][1]+dp[nx][2]
            v2 *= dp[nx][0]+dp[nx][1]+dp[nx][2]*2
        v1 %= MOD
        v2 %= MOD
    if c[now] == 'a':
        dp[now][0] = v1
        dp[now][2] = v2-v1
    if c[now] == 'b':
        dp[now][1] = v1
        dp[now][2] = v2-v1


dp = [[0]*3 for _ in range(N)]

dfs(0, -1)
print((dp[0][2] % MOD + MOD) % MOD)
