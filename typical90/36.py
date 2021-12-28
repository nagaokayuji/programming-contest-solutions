from pprint import pprint
import sys
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


N, Q = mi()
XY = [tuple(mi()) for _ in range(N)]

RXY = []
RXS = []
RYS = []
for x, y in XY:
    RXY.append((x+y, x-y))
    RXS.append(x+y)
    RYS.append(x-y)

RXS.sort()
RYS.sort()

xm = RXS[0]
xM = RXS[-1]
ym = RYS[0]
yM = RYS[-1]
ans = []
for q in [int(input()) for _ in range(Q)]:
    q -= 1
    x, y = RXY[q]
    dm = max(abs(x-xm), abs(xM-x), abs(y-ym), abs(yM-y))
    ans.append(dm)

print(*ans)
