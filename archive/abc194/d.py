N = int(input())


# N-1 は必要
ans = 0
for x in range(1, N):
    ans += N/x
print(ans)
