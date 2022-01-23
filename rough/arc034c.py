import sys
from collections import defaultdict, Counter, deque
from math import sqrt, gcd, factorial, pi, cos, sin, hypot
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


def pf(n):
    ret = defaultdict(int)
    m = n
    for x in range(2, int(sqrt(n)+1.5)):
        while m % x == 0:
            ret[x] += 1
            m //= x
    if m > 1:
        ret[m] += 1
    return ret


def _solve():
    A, B = mi()

    pf_all = defaultdict(int)
    for x in range(B+1, A+1):
        primes = pf(x)
        for key, value in primes.items():
            pf_all[key] += value

    ans = 1
    for key, value in pf_all.items():
        ans *= (value+1)
        ans %= MOD

    print(ans)


if __name__ == '__main__':
    MOD = 10**9+7
    _solve()
