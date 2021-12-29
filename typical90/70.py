from pprint import pprint
import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())
def ti(): return tuple(mi())


N = int(input())
XY = [ti() for _ in range(N)]
X = []
Y = []
for x, y in XY:
    X.append(x)
    Y.append(y)

X.sort()
Y.sort()
xm = X[N//2]
ym = Y[N//2]
ans = 0
for x, y in XY:
    ans += abs(x-xm) + abs(y-ym)
print(ans)
