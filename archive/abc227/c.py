N = int(input())

ans = 0
for a in range(1, N+1):
    if pow(a, 3) > N:
        break
    t = N//a
    for b in range(a, N+1):
        if b*b > t:
            break
        ans += t//b - b + 1
print(ans)
