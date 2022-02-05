import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


def _solve():
    N = int(input())
    abc = [tuple(mi()) for _ in range(N)]

    dp = [0]*3
    for vals in abc:
        ndp = dp.copy()
        for i in range(3):
            for j in range(3):
                if i == j:
                    continue
                ndp[i] = max(ndp[i], dp[j]+vals[i])
        dp = ndp
    print(max(dp))

    return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
