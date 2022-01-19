K = int(input())

ans = 0
for a in range(1, K+1):  # a<=b<=c
    if a**3 > K:
        break
    for b in range(a, K+1):
        mul = a*b
        if mul*b > K:
            break
        for c in range(b, K+1):
            if mul*c > K:
                break
            if a == b == c:
                ans += 1
            elif a == b or b == c:
                ans += 3
            else:
                ans += 6
print(ans)
