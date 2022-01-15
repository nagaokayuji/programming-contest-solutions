n, m = map(int, input().split())
ab = []
for _ in range(m):
    a, b = map(int, input().split())
    ab.append((a-1, b-1))
k = int(input())
cd = []
for _ in range(k):
    c, d = map(int, input().split())
    cd.append((c-1, d-1))

ans = 0
for i in range(2**k):
    balls = [0] * (n)
    tmp = 0
    for j in range(k):
        if (i >> j) & 1 == 1:
            balls[cd[j][0]] += 1
        else:
            balls[cd[j][1]] += 1
    for a, b in ab:
        if balls[a] > 0 and balls[b] > 0:
            tmp += 1
    ans = max(ans, tmp)
print(ans)
