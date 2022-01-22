import sys
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


sys.setrecursionlimit(10**7)


def dfs(v, prv):
    global dp
    for nx, w in g[v]:
        if nx == prv:
            continue
        dp[nx] = max(dp[nx], dp[v] + w)
        dfs(nx, v)


N = int(input())
stw = [tuple(mi()) for _ in range(N-1)]
g = [[] for _ in range(N)]
for s, t, w in stw:
    g[s].append((t, w))
    g[t].append((s, w))


dp = [0] * N
dfs(0, None)

mxv, mx = None, -1
for i, x in enumerate(dp):
    if x > mx:
        mx = x
        mxv = i

dp = [0] * N
dfs(mxv, None)
print(max(dp))
