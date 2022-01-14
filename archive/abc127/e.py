import sys
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


class ModComb:
    def __init__(self, N=2*(10**5), MOD=10**9 + 7):
        self.MOD = MOD
        self.N = N
        self.factorial = [1] * (N+1)

        for i in range(2, N+1):
            self.factorial[i] = self.factorial[i-1]*i % MOD

        self.inv_factorial = [1] * (N+1)
        self.inv_factorial[N] = pow(self.factorial[N], MOD-2, MOD)
        for i in range(N, 1, -1):
            self.inv_factorial[i-1] = self.inv_factorial[i]*i % MOD

    def comb(self, n, k):
        if n < k or n < 0 or k < 0:
            return 0
        return (self.factorial[n] * self.inv_factorial[k] % self.MOD) * self.inv_factorial[n-k] % self.MOD


def _solve():
    N, M, K = mi()
    md = ModComb()
    p = md.comb(N*M-2, K-2)

    x = y = 0
    for i in range(M):
        x += N**2 * i * (M-i)
    for i in range(N):
        y += M**2 * i * (N-i)

    print((x+y)*p % MOD)


if __name__ == '__main__':
    MOD = 10**9+7
    _solve()
