from pprint import pprint
import sys
from bisect import bisect_left, bisect_right
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())
def ti(): return tuple(mi())


N, K, P = mi()

A = li()


def solve(N, K, P, A):
    A1 = A[:N//2]
    n1 = len(A1)
    A2 = A[N//2:]
    n2 = len(A2)

    r1 = [[] for _ in range(K+1)]
    for bits in range(1 << n1):
        bitc = 0
        price = 0
        for i in range(n1):
            if bits >> i & 1:
                bitc += 1
                price += A1[i]
        if bitc > K:
            continue
        r1[bitc].append(price)

    r2 = [[] for _ in range(K+1)]
    for bits in range(1 << n2):
        bitc = 0
        price = 0
        for i in range(n2):
            if bits >> i & 1:
                bitc += 1
                price += A2[i]
        if bitc > K:
            continue
        r2[bitc].append(price)

    for k in range(K+1):
        r1[k].sort()
        r2[k].sort()

    ans = 0
    for k in range(K+1):
        for a in r1[k]:
            ans += bisect_right(r2[K-k], P-a)
    return ans


print(solve(N, K, P, A))
