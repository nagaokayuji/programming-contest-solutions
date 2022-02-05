import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


def _solve():
    H, W = mi()
    A = [input() for _ in range(H)]
    dp = [[0]*W for _ in range(H)]
    dp[0][0] = 1
    for i in range(H):
        for j in range(W):
            dp[i][j] %= MOD
            if j+1 < W and A[i][j+1] == ".":
                dp[i][j+1] += dp[i][j]
            if i+1 < H and A[i+1][j] == ".":
                dp[i+1][j] += dp[i][j]
    print(dp[H-1][W-1] % MOD)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
