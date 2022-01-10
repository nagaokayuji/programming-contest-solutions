A, B, C, X, Y = map(int, map(int, input().split()))

ans = 10**18
for ab_count in range(0, 2*max(X, Y)+2, 2):
    ans = min(ans, ab_count * C + A * max(0, (X-ab_count//2)) +
              B * max(0, (Y-ab_count//2)))

print(ans)
