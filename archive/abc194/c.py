from collections import defaultdict
N = int(input())
A = list(map(int, input().split()))

d = defaultdict(int)

for x in A:
    d[x] += 1


ans = 0
for i in range(-200, 201):
    for j in range(i+1, 201):
        dif = j-i
        ans += dif*dif * d[i] * d[j]
print(ans)
