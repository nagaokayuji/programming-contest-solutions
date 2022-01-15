N = int(input())


def f(a, b):
    return max(len(str(a)), len(str(b)))


ans = 1000
for a in range(1, int(N**0.5 + 1)):
    if N % a == 0:
        b = N//a
        ans = min(ans, f(a, b))
print(ans)
