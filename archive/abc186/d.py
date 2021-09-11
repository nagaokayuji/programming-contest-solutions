n = int(input())
a = list(map(int, input().split()))
a.sort()


ans = 0

difs = [a[i+1] - a[i] for i in range(n-1)]


for i in range(n-1):
    ans += difs[i] * (i+1) * (n-1-i)

print(ans)
