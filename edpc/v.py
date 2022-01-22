import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


def cumprod(a, MOD):
    if not a:
        return a
    n = len(a)
    ret = [a[0]]
    for x in a[1:]:
        ret.append(ret[-1]*x % MOD)
    return ret


def _solve():
    N, M = mi()
    xy = [tuple(mi1()) for _ in range(N-1)]
    g = [[] for _ in range(N)]
    for x, y in xy:
        g[x].append(y)
        g[y].append(x)

    dp1 = [0] * N
    dp2 = [0] * N

    def dfs1(x, prv=-1):
        ret = 1
        for nx in g[x]:
            if nx == prv:
                continue
            dfs1(nx, x)
            ret = (ret*dp1[nx]) % M
        dp1[x] = (ret+1) % M

    dfs1(0)

    def dfs2(x, prv=-1):
        if prv == -1:
            dp2[x] = 1

        dp = list(map(lambda x: 1 if prv == x else dp1[x], g[x]))
        lp = cumprod(dp, M)
        rp = cumprod(dp[::-1], M)[::-1]

        for i, nx in enumerate(g[x]):
            if nx == prv:
                continue
            p = 1
            if i > 0:
                p = (p*lp[i-1]) % M
            if i+1 < len(g[x]):
                p = (p*rp[i+1]) % M
            dp2[nx] = (dp2[x] * p + 1) % M
            dfs2(nx, x)

    ans = [0]*N

    def dfs3(x, prv=-1):
        ans[x] = dp2[x]
        for nx in g[x]:
            if nx == prv:
                continue
            ans[x] = (ans[x]*dp1[nx]) % M
            dfs3(nx, x)

    dfs2(0)
    dfs3(0)

    print("\n".join(map(str, ans)))


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    _solve()
