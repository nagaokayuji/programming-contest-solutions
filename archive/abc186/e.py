import math
T = int(input())


def ext_gcd(a, b):
    if a == 0:
        return (0, 1, b)
    else:
        (x1, y1, gcd) = ext_gcd(b % a, a)
        return (y1-b//a*x1, x1, gcd)


def solve():
    N, S, K = map(int, input().split())
    gcd = math.gcd(math.gcd(N, S), K)

    N //= gcd
    S //= gcd
    K //= gcd

    x, y, g = ext_gcd(K, N)
    if g == 1:
        invk = x
        print(((-invk * S) % N + N*8) % N)

    else:
        print(-1)


for _ in range(T):
    solve()
