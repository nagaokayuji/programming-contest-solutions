from collections import *
import sys
sys.setrecursionlimit(1111111)

N = int(input())

g = [[] for _ in range(N)]

for _ in range(N-1):
    a, b = map(int, input().split())
    a -= 1
    b -= 1
    g[a].append(b)
    g[b].append(a)


for v in g:
    v.sort()

visited = [False]*N

st = [0]


def dfs(now, prev):
    print(now+1)
    visited[now] = True
    for nx in g[now]:
        if not visited[nx]:
            dfs(nx, now)
            print(now+1)


dfs(0, 0)
