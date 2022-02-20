import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def li(): return list(map(int, input().split()))
def li1(): return list(map(lambda x: int(x)-1, input().split()))
def yn(a): print("Yes" if a else "No")


def _solve():
    ''''''
    N, = li()
    UV = [li1() for _ in range(N-1)]
    g = [[] for _ in range(N)]
    for u, v in UV:
        g[u].append(v)
        g[v].append(u)

    anss = [(INF, 0)] * N

    leafc = 0

    def dfs(v, prev=-1):
        nonlocal leafc
        sz = 1
        l = INF
        r = 0
        for to in g[v]:
            if to == prev:
                continue
            dfs(to, v)
            sz += 1
            ll, rr = anss[to]
            l = min(l, ll)
            r = max(r, rr)

        if sz == 1:
            leafc += 1
            anss[v] = (leafc, leafc)
        else:
            anss[v] = (l, r)
    dfs(0)

    for l, r in anss:
        print(l, r)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
