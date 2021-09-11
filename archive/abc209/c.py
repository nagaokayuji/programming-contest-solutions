n = int(input())

c = list(map(int, input().split()))

MOD = 10**9 + 7
c.sort()

ans = 1
for i in range(n):
    ans *= max(0, (c[i] - i))
    ans %= MOD

print(ans)
