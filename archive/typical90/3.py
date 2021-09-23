N = int(input())
AB = [tuple(map(int, input().split())) for _ in range(N-1)]

g = [[] for _ in range(N)]
for a, b in AB:
    a -= 1
    b -= 1
    g[a].append(b)
    g[b].append(a)


def l(fr):
    mx = 0
    mxv = 0
    ls = [(fr, 0)]
    visited = [False]*N
    visited[fr] = True
    while ls:
        now, cnt = ls.pop()
        visited[now] = True
        if cnt > mx:
            mx = cnt
            mxv = now
        for nx in g[now]:
            if not visited[nx]:
                ls.append((nx, cnt+1))

    return mx, mxv


a1, b1 = l(0)
print(l(b1)[0] + 1)
