import sys
from pprint import pprint
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def inputmi(): return map(int, input().split())
def inputmi1(): return map(lambda x: int(x)-1, input().split())
def inputli(): return list(inputmi())
def inputli1(): return list(inputmi1())
def inputti(): return tuple(inputmi())
def inputti1(): return tuple(inputmi1())
def inputi(): return int(input())


def _solve():
    N, M, S, T = inputti()
    S -= 1
    T -= 1
    UVAB = [inputti() for _ in range(M)]
    g = [[]for _ in range(N)]
    for u, v, a, b in UVAB:
        u -= 1
        v -= 1
        g[u].append((v, a, b))
        g[v].append((u, a, b))

    def dijkstra(g, start, mn):
        dists = [10**18] * N
        dists[start] = 0
        que = [(0, start)]
        while que:
            p = heappop(que)
            c = p[0]
            v = p[1]
            if dists[v] < c:
                continue
            for to, cost_y, cost_s in g[v]:
                cost = cost_y if mn else cost_s
                if dists[to] > dists[v] + cost:
                    dists[to] = dists[v] + cost
                    heappush(que, (dists[to], to))
        return dists

    from_s_y = dijkstra(g, S, True)
    from_t_s = dijkstra(g, T, False)
    ans = [INF]
    for i in range(N-1, -1, -1):
        ans.append(min(ans[-1], from_s_y[i]+from_t_s[i]))
    yen = 10**15
    for d in ans[N:0:-1]:
        print(yen-d)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
