import sys
from math import sqrt, gcd, factorial, atan2, degrees
from bisect import bisect_left, bisect_right
import numpy as np
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())


N = int(input())
C1 = [0]*N
C2 = [0]*N
for i in range(N):
    c, p = mi()
    if c == 1:
        C1[i] = p
    else:
        C2[i] = p

C1s = np.cumsum([0] + C1)
C2s = np.cumsum([0] + C2)

Q = int(input())
for _ in range(Q):
    l, r = mi()
    print(C1s[r]-C1s[l-1], C2s[r]-C2s[l-1])
