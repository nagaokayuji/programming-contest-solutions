import sys
from pprint import pprint
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


def _solve():
    N, W = mi()
    wv = [tuple(mi()) for _ in range(N)]
    dp = [0]*(W+1)
    for w, v in wv:
        for pw in range(W-w, -1, -1):
            dp[pw+w] = max(dp[pw+w], dp[pw]+v)
    print(dp[-1])


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
