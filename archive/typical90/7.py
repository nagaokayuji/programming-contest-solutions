import sys
from bisect import bisect_left, bisect_right
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


N = int(input())
A = sorted(li())
Q = int(input())
INF = 10**17

for _ in range(Q):
    B = int(input())
    i = bisect_right(A, B)
    a, b = INF, INF
    if i < N:
        a = abs(A[i]-B)
    if i > 0:
        b = abs(A[i-1]-B)
    print(min(a, b))
