from collections import defaultdict
import bisect
import heapq

INF = 10**20


def solve():
    N = int(input())
    LR = []
    for _ in range(N):
        L, R = map(int, input().split())
        LR.append((L, R))
    LR.sort()
    LR.append((INF, INF))
    pq = []  # priority queue

    x = 1

    for (l, r) in LR:
        while x < l and pq:
            if (heapq.heappop(pq) < x):
                print("No")
                return
            x += 1
        x = l
        heapq.heappush(pq, r)
    print("Yes")


T = int(input())

for _ in range(T):
    solve()
