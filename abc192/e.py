from collections import deque
import sys
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
def input(): return sys.stdin.readline().rstrip()


N, M, X, Y = map(int, input().split())
X -= 1
Y -= 1

INF = 10**15 + 5
dp = [INF]*N
dp[X] = 0

g = [[] for _ in range(N)]
for _ in range(M):
    a, b, t, k = map(int, input().split())
    a -= 1
    b -= 1
    g[a].append((b, t, k))
    g[b].append((a, t, k))

q = []
heappush(q, (0, X))
while q:
    now_t, now = heappop(q)
    if dp[now] < now_t:
        continue
    dp[now] = now_t

    for (v, t, k) in g[now]:
        if dp[v] != INF:
            continue
        nx_t = t + (k*((now_t+k-1)//k))
        heappush(q, (nx_t, v))

ans = dp[Y]
if ans == INF:
    print(-1)
else:
    print(ans)
