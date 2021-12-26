from pprint import pprint
import sys
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


N = int(input())
AB = [tuple(mi()) for _ in range(N-1)]


g = [[] for _ in range(N)]

for a, b in AB:
    g[a-1].append(b-1)
    g[b-1].append(a-1)


f = [False]*N

visited = [False]*N
evs = []
ods = []

s = [(0, 0)]
while s:
    now, depth = s.pop()
    if depth % 2 == 0:
        evs.append(now)
    else:
        ods.append(now)
    visited[now] = True
    for nx in g[now]:
        if not visited[nx]:
            s.append((nx, depth+1))

ans = evs if len(evs) >= len(ods) else ods

for x in ans[:N//2]:
    print(x+1)
