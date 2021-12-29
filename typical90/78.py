from pprint import pprint
import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())
def ti(): return tuple(mi())
def i(): return int(input())


N, M = mi()
ab = [ti() for _ in range(M)]

g = [[] for _ in range(N)]
for a, b in ab:
    a -= 1
    b -= 1
    g[a].append(b)
    g[b].append(a)

ans = 0
for i in range(N):
    c = 0
    for j in g[i]:
        if i > j:
            c += 1
    if c == 1:
        ans += 1
print(ans)
