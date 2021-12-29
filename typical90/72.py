import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())
def ti(): return tuple(mi())


H, W = mi()
C = [list(input()) for _ in range(H)]

if H <= 1 or W <= 1:
    print(-1)
    exit()

_dx = [1, 0, 0, -1]
_dy = [0, -1, 1, 0]

ans = 0


def dfs(v, visitedlist, st):
    global ans
    if v in visitedlist:
        return
    x, y = v
    for dx, dy in zip(_dx, _dy):
        nx, ny = x+dx, y+dy
        if 0 <= nx < H and 0 <= ny < W and C[nx][ny] == '.':
            if (nx, ny) == st:
                ans = max(ans, len(visitedlist)+1)
            else:
                l = visitedlist.copy()
                l.append(v)
                dfs((nx, ny), l, st)


for i in range(H):
    for j in range(W):
        if C[i][j] == ".":
            dfs((i, j), [], (i, j))

if ans > 2:
    print(ans)
else:
    print(-1)
