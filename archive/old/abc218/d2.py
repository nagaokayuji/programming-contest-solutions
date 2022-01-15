N = int(input())
xy = [tuple(map(int, input().split())) for _ in range(N)]

xs = sorted(set(map(lambda x: x[0], xy)))
ys = sorted(set(map(lambda x: x[1], xy)))
xmap = {x: i for i, x in enumerate(xs)}
ymap = {y: i for i, y in enumerate(ys)}
xy = list(map(lambda x: (xmap[x[0]], ymap[x[1]]), xy))
xy.sort()

exists = [[False] * (N+1) for _ in range(N+1)]

for x, y in xy:
    exists[x][y] = True

ans = 0

for i in range(N):
    x, y = xy[i]
    for j in range(i+1, N):
        x2, y2 = xy[j]
        if not (x < x2 and y < y2):
            continue
        if exists[x][y2] and exists[x2][y]:
            ans += 1

print(ans)
