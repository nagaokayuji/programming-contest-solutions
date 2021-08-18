import sys
sys.setrecursionlimit(10**7)
N, Q = map(int, input().split())
ab = [tuple(map(int, input().split())) for _ in range(N-1)]
qx = [tuple(map(int, input().split())) for _ in range(Q)]

g = [[] for _ in range(N)]

for a, b in ab:
    g[a-1].append(b-1)
    g[b-1].append(a-1)

cnts = [0] * N


def dfs(now, prev):
    cnt = 1
    for nx in g[now]:
        if nx == prev:
            continue
        cnt += dfs(nx, now)
    cnts[now] = cnt
    return cnt


dfs(0, -1)

ans = 0
for q, x in qx:
    ans += cnts[q-1] * x
    print(ans)
