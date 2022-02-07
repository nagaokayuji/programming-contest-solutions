N, M = map(int, input().split())

# M no ykusu
div = set()
m = M
for i in range(1, int(m**0.5) + 1):
    if m % i == 0:
        div.add(i)
        div.add(m//i)
div = sorted(list(div))

ans = 1
for d in div:
    if d*N <= M:
        ans = max(ans, d)
print(ans)
