import sys
from math import sqrt, gcd, factorial, atan2, degrees
from bisect import bisect_left, bisect_right
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


N = int(input())
XY = [tuple(mi()) for _ in range(N)]

ans = 0


def solve(pos):
    vec = []
    for i in range(N):
        if i == pos:
            continue
        sa = (XY[i][0] - XY[pos][0], XY[i][1] - XY[pos][1])
        th = atan2(sa[1], sa[0])
        degree = degrees(th)
        vec.append(degree)
    vec.sort()
    ret = 0.0
    for deg in vec:
        target = deg+180.0
        if target >= 360:
            target -= 360
        lb = bisect_left(vec, target)
        idx1 = lb % len(vec)
        idx2 = (lb+len(vec)-1) % len(vec)
        d1 = abs(deg-vec[idx1])
        d2 = abs(deg-vec[idx2])
        if d1 >= 180.0:
            d1 = 360.0-d1
        if d2 >= 180.0:
            d2 = 360.0-d2

        ret = max(ret, d1, d2)
    return ret


for i in range(N):
    ans = max(ans, solve(i))

print(ans)
