from pprint import pprint
import sys
from collections import defaultdict, Counter, deque
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())
def ti(): return tuple(mi())


Q = int(input())
TX = [ti() for _ in range(Q)]
d = deque()

for t, x in TX:
    if t == 1:
        d.appendleft(x)
    if t == 2:
        d.append(x)
    if t == 3:
        print(d[x-1])
