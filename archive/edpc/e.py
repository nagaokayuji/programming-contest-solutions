import sys
from pprint import pprint
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


def _solve():
    N, W = mi()
    wv = [tuple(mi()) for _ in range(N)]
    mxv = 100100
    dp = [INF]*(mxv+2)
    dp[0] = 0

    for w, v in wv:
        # for pv in range(1000, -1, -1):
        for pv in range(mxv-v, -1, -1):
            if pv+v > mxv:
                continue
            dp[pv+v] = min(dp[pv+v], dp[pv]+w)

    ans = 0
    for i, x in enumerate(dp):
        if x <= W:
            ans = i

    print(ans)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
