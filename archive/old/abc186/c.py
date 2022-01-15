n = int(input())


def f(n):
    def ten(n):
        while n:
            if n % 10 == 7:
                return False
            n //= 10
        return True

    def eight(n):
        while n:
            if n % 8 == 7:
                return False
            n //= 8
        return True

    return ten(n) and eight(n)


ans = 0
for i in range(1, n+1):
    if f(i):
        ans += 1
print(ans)
