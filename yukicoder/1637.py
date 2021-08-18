N, Q = map(int, input().split())
ab = [tuple(map(int, input().split())) for _ in range(N-1)]
qx = [tuple(map(int, input().split())) for _ in range(Q)]

g = [[] for _ in range(N)]

for a, b in ab:
    g[a-1].append(b-1)
    g[b-1].append(a-1)

ans = 0
for q, x in qx:
    ans += N * x
    print(ans)
