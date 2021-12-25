import sys
from math import sqrt, gcd, factorial
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


N, K = mi()
A = li()
B = li()


dif = 0
for a, b in zip(A, B):
    dif += abs(a-b)


if dif > K:
    print("No")
else:
    if (K-dif) % 2 == 0:
        print("Yes")
    else:
        print("No")
