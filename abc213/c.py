
h, w, n = map(int, input().split())
ab = [tuple(map(int, input().split())) for _ in range(n)]

mapa = {}
mapb = {}

for i, x in enumerate(sorted(set(map(lambda x: x[0], ab)))):
    mapa[x] = i+1

for i, x in enumerate(sorted(set(map(lambda x: x[1], ab)))):
    mapb[x] = i+1

for a, b in ab:
    print(mapa[a], mapb[b])
