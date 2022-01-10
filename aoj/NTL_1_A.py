N = int(input())
n = N

ans = []
for x in range(2, int(n**0.5)+1):
    while n % x == 0:
        ans.append(x)
        n //= x
if n > 1:
    ans.append(n)

print(f"{N}:", *ans)
