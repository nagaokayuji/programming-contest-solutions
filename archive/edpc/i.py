import sys
from pprint import pprint
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


def _solve():
    N = int(input())
    P = list(map(float, input().split()))

    dp = [1]
    for i, p in enumerate(P):
        ndp = [0]*(i+2)
        for j in range(i+1):
            ndp[j+1] += dp[j]*p
            ndp[j] += dp[j]*(1-p)
        dp = ndp.copy()

    print(sum(dp[N//2+1:]))


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
