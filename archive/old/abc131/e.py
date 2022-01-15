N, K = map(int, input().split())

ans = []

for i in range(1, N):
    ans.append((0, i))
k = (N-1)*(N-2)//2
if k < K:
    print(-1)
    exit()

for i in range(1, N):
    for j in range(i+1, N):
        if k == K:
            break
        ans.append((i, j))
        k -= 1

print(len(ans))
for a, b in ans:
    print(a+1, b+1)
