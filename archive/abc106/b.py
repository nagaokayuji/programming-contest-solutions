N = int(input())


def get(n):
    yakusu = set()
    for x in range(1, int(n**0.5)+1):
        if n % x == 0:
            yakusu.add(x)
            yakusu.add(n//x)
    return len(yakusu)


ans = 0
for i in range(1, N+1, 2):
    if get(i) == 8:
        ans += 1
print(ans)
