from collections import deque
import sys
def input(): return sys.stdin.readline().rstrip()


N, M, X, Y = map(int, input().split())
X -= 1
Y -= 1

INF = 10**9 + 5
dp = [INF]*N
dp[X] = 0
abtk = [list(map(int, input().split())) for _ in range(M)]

g = [[] for _ in range(N)]
for _a, _b, t, k in abtk:
    a = _a-1
    b = _b-1
    g[a].append((b, t, k))
    g[b].append((a, t, k))

l = deque()
l.append(X)
while l:
    now = l.popleft()
    now_t = dp[now]

    for (v, t, k) in g[now]:
        nx_t = t + (k*((now_t+k-1)//k))
        if nx_t < dp[v]:
            dp[v] = nx_t
            l.append(v)

ans = dp[Y]
if ans == INF:
    print(-1)
else:
    print(ans)
