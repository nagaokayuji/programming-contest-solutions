import sys
from pprint import pprint
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


def _solve():
    N = int(input())
    xy = [tuple(mi1()) for _ in range(N-1)]
    g = [[] for _ in range(N)]
    for x, y in xy:
        g[x].append(y)
        g[y].append(x)

    dp = [[None]*2 for _ in range(N)]  # 0: white, 1: black

    def dfs(v,  color, prv):
        if dp[v][color] is not None:
            return dp[v][color]

        ret = 1
        for nx in g[v]:
            if nx == prv:
                continue
            if color == 0:  # white
                ret *= dfs(nx, 0, v) + dfs(nx, 1, v)
            else:
                ret *= dfs(nx, 0, v)
            ret %= MOD
        if ret == 0:
            ret = 1
        dp[v][color] = ret
        return ret

    dfs(0, 0, -1)
    dfs(0, 1, -1)
    pprint((dp[0][0]+dp[0][1]) % MOD)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
