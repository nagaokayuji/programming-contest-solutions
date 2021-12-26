import sys
from bisect import bisect_left, bisect_right
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


N, M = mi()

A = sorted(li())
B = sorted(li())

ans = 10**10
for a in A:
    i = bisect_left(B, a)
    ans = min(ans, abs(a-B[i % M]), abs(a-B[i-1]))

print(ans)
