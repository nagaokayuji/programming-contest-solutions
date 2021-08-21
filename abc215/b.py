n = int(input())

ans = 0
val = 1

while val <= n:
    val *= 2
    ans += 1
print(ans-1)
