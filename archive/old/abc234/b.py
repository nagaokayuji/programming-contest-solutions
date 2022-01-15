import sys
from math import sqrt, gcd, factorial, pi, cos, sin, hypot
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())


N = int(input())
XY = [tuple(mi()) for _ in range(N)]
ans = 0
for x, y in XY:
    for x2, y2 in XY:
        ans = max(ans, hypot(x-x2, y-y2))
print(ans)
