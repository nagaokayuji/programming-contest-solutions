import sys
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())


N, K = mi()
MOD = 10**9+7
# 連続する3個は別の色


if N < 1000:
    ret = 1
    for i in range(N):
        ret *= K - min(i, 2)
        ret %= MOD
    print(ret % MOD)
    exit()

print(K*(K-1)*pow(K-2, N-2, MOD) % MOD)
