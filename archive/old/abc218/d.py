import bisect
N = int(input())
xy = [tuple(map(int, input().split())) for _ in range(N)]

# asshuku
xs = sorted(set(map(lambda x: x[0], xy)))
ys = sorted(set(map(lambda x: x[1], xy)))
xmap = {x: i for i, x in enumerate(xs)}
ymap = {y: i for i, y in enumerate(ys)}
xy = list(map(lambda x: (xmap[x[0]], ymap[x[1]]), xy))
yx = list(map(lambda x: (x[1], x[0]), xy))
# done
xy.sort()
yx.sort()
xmax = xy[-1][0]
ymax = yx[-1][0]


xx = [[] for _ in range(xmax+1)]
for x, y in xy:
    xx[x].append(y)

yy = [[] for _ in range(ymax+1)]
for x, y in xy:
    yy[y].append(x)

ans = 0


for x in range(0, xmax+1):
    ys = xx[x]
    r = len(ys)
    ycnt = [0] * (xmax+1)
    for y in ys:
        for xxx in yy[y]:
            if xxx > x:
                ycnt[xxx] += 1
    for i in range(x+1, xmax+1):
        if ycnt[i] >= 2:
            k = min(r, ycnt[i])
            ans += k*(k-1)//2
    # for y in ys:
    #     i = bisect.bisect(y, x)
    #     ans += r * len(len(y) - i)

print(ans)
