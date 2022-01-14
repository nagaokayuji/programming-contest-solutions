import sys
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


def _solve():
    N = int(input())
    uvw = [tuple(mi()) for _ in range(N-1)]
    g = [[] for _ in range(N)]
    for u, v, w in uvw:
        u -= 1
        v -= 1
        g[u].append((v, w))
        g[v].append((u, w))

    dists = [None]*N
    stack = [(0, 0, None)]
    while stack:
        now, dist, prev = stack.pop()
        dists[now] = dist % 2
        for (nx, w) in g[now]:
            if nx == prev:
                continue
            stack.append((nx, dist+w, now))
    print(*dists)


if __name__ == '__main__':
    _solve()
