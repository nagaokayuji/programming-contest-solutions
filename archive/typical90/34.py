from pprint import pprint
import sys
from collections import defaultdict, Counter, deque
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


N, K = mi()
A = li()

di = defaultdict(int)

ans = 0
l = 0
r = 1
di[A[0]] += 1
k = 1
while l < N:
    while r < N and k <= K:  # add r
        if di[A[r]] == 0:
            if k == K:
                break
            else:
                k += 1
        di[A[r]] += 1
        r += 1
    ans = max(ans, r-l)

    di[A[l]] -= 1
    if di[A[l]] == 0:
        k -= 1

    l += 1

print(ans)
