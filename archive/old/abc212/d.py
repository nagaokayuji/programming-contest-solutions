from heapq import heappush, heappop, heapify, heappushpop, heapreplace
import sys
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


Q = int(input())
q = []
add = 0
for _ in range(Q):
    t = li()
    if t[0] == 1:  # x を追加
        x = t[1]
        heappush(q, x-add)
    elif t[0] == 2:  # すべてに x を追加
        x = t[1]
        add += x
    else:
        v = heappop(q)
        print(v+add)
