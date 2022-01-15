from collections import deque
import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


def _solve():
    N, M = mi()
    uv = [tuple(mi1()) for _ in range(M)]
    g = [[] for _ in range(N)]
    for u, v in uv:
        g[u].append(v)
    S, T = mi1()

    dp = [[INF]*3 for _ in range(N)]

    # stack = [(S, 0)]
    q = deque()
    q.append((S, 0))
    while q:
        v, dist = q.popleft()

        for nx in g[v]:
            if dp[nx][(dist+1) % 3] <= dist+1:
                continue
            dp[nx][(dist+1) % 3] = dist+1
            q.append((nx, dist+1))

    if dp[T][0] == INF:
        print(-1)
    else:
        print(dp[T][0]//3)


if __name__ == '__main__':
    _solve()
