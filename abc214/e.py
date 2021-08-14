from collections import defaultdict
import bisect
import heapq

INF = 10**20


def solve():
    N = int(input())
    LR = defaultdict(list)

    for _ in range(N):
        L, R = list(map(int, input().split()))
        LR[L].append(R)

    Ls = list(LR.keys()) + [INF]
    Ls.sort()

    l = 1
    c = 0
    Q = []
    while c != N:
        # search index
        i = bisect.bisect_left(Ls, l)
        if Ls[i] == l:
            for r in LR[l]:
                heapq.heappush(Q, r)

        if not Q:
            l = Ls[bisect.bisect_left(Ls, l)]
            continue

        r = heapq.heappop(Q)
        if r < l:
            break

        l += 1
        c += 1

    if c == N:
        print("Yes")
    else:
        print("No")


T = int(input())

for _ in range(T):
    solve()
