from pprint import pprint
import sys
from collections import defaultdict, Counter, deque
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())
def ti(): return tuple(mi())


MOD = 10**9+7
N = int(input())
A = [li() for _ in range(N)]

ans = 1
for r in A:
    ans *= sum(r)
    ans %= MOD
print(ans)
